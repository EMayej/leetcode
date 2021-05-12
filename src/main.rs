use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::fs;
use std::io::Write;
use std::path::Path;

mod fetcher;
mod template;

mod problem;

const TITLE_SLUG: &str = "TITLE_SLUG";
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
    let problem = fetcher::fetch_with_slug(slug);
    let file_name = format!(
        "problem_{:0>4}_{}",
        problem.id,
        slug.replace("-", "_"),
    );
    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("problem already initialized");
    }

    let source = template::render(&problem);
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
    let _ = writeln!(lib_file, "mod {};", file_name);
}

