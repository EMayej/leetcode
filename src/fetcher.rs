use html2text;
use serde::{Deserialize, Serialize};

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

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        codeDefinition
        questionFrontendId
        title
    }
}"#;

use crate::problem::Problem;

pub fn fetch_with_slug(slug: &str) -> Problem {
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
    let question = response.data.question;

    let link = format!("https://leetcode.com/problems/{}", slug);
    let content = normalize(&question.content);
    let code = question
        .code_definition
        .into_iter()
        .find(|code| code.value == "rust")
        .unwrap()
        .default_code;
    Problem {
        id: question.question_frontend_id,
        title: question.title,
        content,
        code,
        link,
    }
}

fn normalize(content: &str) -> String {
    html2text::from_read(content.as_bytes(), 80).replace("\n", "\n *")
}
