use super::judgement::Requirement;
use colored::Colorize;

pub fn display_result(
    requirements: &Requirement,
    prefix: &str,
    children_prefix: &str,
    pat: &str,
    show_regex: &bool,
) {
    print!("{}{} ", prefix, requirements.label);

    let result =
        requirements.acquired_credit.to_string() + "/" + requirements.credit.to_string().as_str();

    match requirements.ok {
        true => println!("{}", result.green()),
        false => println!("{}", result.red()),
    }

    if *show_regex && requirements.filter.is_some() {
        println!(
            "{}{}",
            pat,
            requirements.filter.as_ref().unwrap().regex.yellow()
        );
    }

    let mut iter = requirements.children.iter().peekable();

    while let Some(child) = iter.next() {
        match iter.peek().is_some() {
            true => {
                let child_prefix = format!("{}{}", children_prefix, "├── ");
                let child_children_prefix = format!("{}{}", children_prefix, "│   ");
                display_result(
                    child,
                    &child_prefix,
                    &child_children_prefix,
                    &child_children_prefix,
                    show_regex,
                );
            }
            false => {
                let child_prefix = format!("{}{}", children_prefix, "└── ");
                let child_children_prefix = format!("{}{}", children_prefix, "    ");
                let pat = format!("{}{}", children_prefix, "    ");
                display_result(
                    child,
                    &child_prefix,
                    &child_children_prefix,
                    &pat,
                    show_regex,
                );
            }
        }
    }
}
