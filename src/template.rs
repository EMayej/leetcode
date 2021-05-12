use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_template(
            "problem",
            r#"/**
 *
 * {{ problem.link }}
 *
 * {{ problem.id }}. {{ problem.title }}
 *
 * {{ problem.content }}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_{{ problem.id }}() {
        for case in vec![
        ] {
        }
    }
}

struct Solution {}

// {{ problem.link }}
// START SOLUTION
{{ problem.code }}
"#,
        ) {
            eprintln!("fail to init template: {}", e);
            ::std::process::exit(1);
        };
        tera
    };
}

use crate::problem::Problem;

pub fn render(problem: &Problem) -> String {
    let mut context = tera::Context::new();
    context.insert("problem", &problem);
    TEMPLATES.render("problem", &context).unwrap()
}
