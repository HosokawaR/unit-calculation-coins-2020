extern crate toml;
use super::judgement::Filter;
use super::judgement::FilterType;
use super::judgement::Requirement;
use serde::de::value::Error;
use toml::value::Table;

use std::fs;

fn get_filter(requirements: Table) -> Option<Filter> {
    match requirements.get("filter") {
        Some(filter) => filter.as_table().map(|filter| {
            let filter_kind = filter
                .get("type")
                .expect("Failed to get filter type.")
                .as_str()
                .unwrap();
            Filter {
                kind: match filter_kind {
                    "name" => FilterType::Name,
                    "code" => FilterType::Code,
                    _ => panic!("{} is unknown filter type", filter_kind),
                },
                regex: filter
                    .get("regex")
                    .expect("Failed to get regex.")
                    .as_str()
                    .unwrap()
                    .to_string(),
            }
        }),
        None => None,
    }
}

fn build_requiremnts<'a>(requirements: Table, label: String) -> Result<Requirement<'a>, Error> {
    let children = requirements
        .clone()
        .into_iter()
        .filter(|requirement| {
            requirement.0 != "credit"
                && requirement.0 != "filter"
                && requirement.0 != "order"
                && requirement.0 != "max"
        })
        .map(|requirement| {
            let label = requirement.0;
            let _label = label.clone();
            let table = requirement.1.as_table().unwrap().clone();
            build_requiremnts(table, label)
                .expect(format!("Fail to build {} requirement", _label).as_str())
        })
        .collect();

    let credit = requirements
        .get("credit")
        .expect(format!("Failed to get credit at \n{:#?}.", requirements).as_str())
        .as_float()
        .unwrap();

    let limit_credit = match requirements.get("max") {
        Some(order) => order.as_float().unwrap(),
        None => credit,
    };

    let order = match requirements.get("order") {
        Some(order) => Some(order.as_integer().unwrap()),
        None => None,
    };

    let filter = get_filter(requirements);

    Ok(Requirement {
        label,
        credit,
        limit_credit,
        filter,
        children,
        acquired_credit: 0.0,
        ok: false,
        followed_by: "",
        order,
    })
}

pub fn read_toml(input_path: &str) -> Requirement {
    let content = fs::read_to_string(input_path).expect("Failed to read toml file");

    let settings = toml::from_str::<Table>(content.as_str()).expect("Failed to parse toml file");

    build_requiremnts(settings, "全体".to_string())
        .expect("Failed to build requirements from toml file.")
}
