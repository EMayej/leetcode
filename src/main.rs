use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::fs;
use std::io::Write;
use std::path::Path;

use leetcode::fetcher;
use leetcode::template;

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
    let problem_mod_name = format!("problem_{:0>4}_{}", problem.id, slug.replace("-", "_"));
    {
        let problem_file_path = Path::new("./src/problem").join(format!("{}.rs", problem_mod_name));
        if problem_file_path.exists() {
            panic!("problem {} exists", problem_file_path.display());
        }
        let mut problem_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&problem_file_path)
            .unwrap();
        problem_file
            .write_all(template::render(&problem).as_bytes())
            .unwrap();
    }
    {
        let mut mod_problem = fs::OpenOptions::new()
            .append(true)
            .open("./src/problem.rs")
            .unwrap();
        let _ = writeln!(mod_problem, "mod {};", problem_mod_name);
    }
}
