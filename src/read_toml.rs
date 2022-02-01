extern crate toml;

use serde::{de::value::Error, Deserialize};
use toml::value::Table;

use super::judgement::Filter;
use super::judgement::FilterType;
use super::judgement::Requirement;

use std::borrow::BorrowMut;
use std::fs;

fn get_filter(requirements: &Table) -> Option<Filter> {
    match requirements.get("filter") {
        Some(filter) => filter.as_table().map(|filter| {
            let filter_kind = filter.get("type").unwrap().as_str().unwrap();
            Filter {
                kind: match filter_kind {
                    "name" => FilterType::Name,
                    "code" => FilterType::Code,
                    _ => panic!("{} is unknown filter type", filter_kind),
                },
                regex: filter.get("regex").unwrap().as_str().unwrap(),
            }
        }),
        None => None,
    }
}

fn build_requiremnts<'a>(
    requirements: &'a Table,
    label: &'a str,
) -> Result<Requirement<'a>, Error> {
    let children = requirements
        .into_iter()
        .filter(|setting| setting.0 != "credit" && setting.0 != "filter")
        .map(|setting| {
            let table = setting.1.as_table().unwrap();
            build_requiremnts(table, setting.0)
                .expect(format!("Fail to build {} requirement", label).as_str())
        })
        .collect();

    let credit = requirements.get("credit").unwrap().as_float().unwrap();
    let filter = get_filter(requirements);

    Ok(Requirement {
        label,
        credit,
        filter,
        children,
        acquired_credit: 0.0,
        ok: false,
        followed_by: "",
    })
}

pub fn read_toml(input_path: &str) {
    let content = fs::read_to_string(input_path).expect("Failed to read toml file");

    let settings = toml::from_str::<Table>(content.as_str()).expect("Failed to parse toml file");

    let requirement =
        build_requiremnts(&settings, "全体").expect("Failed to build requirements from toml file.");

    println!("{:#?}", requirement);
}
