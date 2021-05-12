use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use html2text;
use lazy_static::lazy_static;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;
use tera::Tera;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_template(
            "problem",
            r#"
/**
 *
 * {{ link }}
 *
 * {{ id }}. {{ title }}
 *
 * {{ content }}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__{{ id }} () {
        for case in vec![
        ] {

        }
    }
}

struct Solution {}

/// START SOLUTION
{{ code }}
"#,
        ) {
            eprintln!("fail to init template: {}", e);
            ::std::process::exit(1);
        };
        tera
    };
}

mod problem;

const TITLE_SLUG: &str = "TITLE_SLUG";

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

#[derive(Serialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    variables: serde_json::Value,
    query: &'static str,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Deserialize)]
struct Question {
    content: String,

    #[serde(rename = "questionFrontendId")]
    question_frontend_id: String,

    #[serde(rename = "codeDefinition", with = "serde_with::json::nested")]
    code_definition: Vec<CodeDefinition>,

    title: String,
}

#[derive(Debug, Deserialize)]
struct CodeDefinition {
    value: String,

    #[serde(rename = "defaultCode")]
    default_code: String,
}

const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        codeDefinition
        questionFrontendId
        title
    }
}"#;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name(TITLE_SLUG)
                .short("s")
                .long("slug")
                .value_name("SLUG")
                .help("title slug")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let slug = matches.value_of(TITLE_SLUG).unwrap();

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(GRAPHQL_URL)
        .json(&Query {
            operation_name: "questionData",
            variables: serde_json::json!({ "titleSlug": slug }),
            query: QUESTION_QUERY_STRING,
        })
        .send()
        .unwrap()
        .json::<Response>()
        .unwrap();
    let code = response
        .data
        .question
        .code_definition
        .iter()
        .find(|code| code.value == "rust")
        .unwrap();

    let mut context = tera::Context::new();
    context.insert("code", &code.default_code);
    context.insert("id", &response.data.question.question_frontend_id);
    context.insert("title", &response.data.question.title);
    context.insert("content", &normalize(&response.data.question.content));
    let link = format!("https://leetcode.com/problems/{}", "two-sum");
    context.insert("link", &link);
    let source = TEMPLATES.render("problem", &context).unwrap();

    let file_name = format!(
        "problem_{:0>4}_{}",
        response.data.question.question_frontend_id,
        slug.replace("-", "_"),
    );
    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("problem already initialized");
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    let mut lib_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/problem.rs")
        .unwrap();
    writeln!(lib_file, "mod {};", file_name);
}

fn normalize(content: &str) -> String {
    html2text::from_read(content.as_bytes(), 80).replace("\n", "\n *")
}
