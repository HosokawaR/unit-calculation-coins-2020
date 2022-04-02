extern crate getopts;
use regex::Regex;
use serde::Deserialize;

use crate::judgement::{Filter, FilterType};

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "学籍番号")]
    pub student_number: String,
    #[serde(rename = "学生氏名")]
    pub student_name: String,
    #[serde(rename = "科目番号")]
    pub course_code: String,
    #[serde(rename = "科目名")]
    pub course_name: String,
    // csv の単位数カラムに余分な空白があり serde の標準機能では f64 に変換できないため独自の変換関数を用意
    #[serde(rename = "単位数 ", deserialize_with = "deserialize_credit")]
    pub credit: f64,
    #[serde(rename = "春学期")]
    pub spring: String,
    #[serde(rename = "秋学期")]
    pub fall: String,
    #[serde(rename = "総合評価")]
    pub overall_grade: String,
    #[serde(rename = "科目区分")]
    pub course_category: String,
    #[serde(rename = "開講年度")]
    pub semester_year: u32,
    #[serde(rename = "開講区分")]
    pub semester_status: String,
    #[serde(default = "bool::default")]
    pub read: bool,
}

impl Record {
    pub fn is_read(&self) -> bool {
        self.read
    }

    pub fn set_read(&mut self, read: bool) {
        self.read = read;
    }

    pub fn is_match(&self, filter: &Filter) -> bool {
        let regex = Regex::new(filter.regex.as_str()).unwrap();
        regex.is_match(match filter.kind {
            FilterType::Code => self.course_code.as_str(),
            FilterType::Name => self.course_name.as_str(),
        })
    }

    pub fn is_acquired(&self) -> bool {
        let regex = Regex::new(r"^A\+|[A-C]|P$").unwrap();
        regex.is_match(self.overall_grade.as_str())
    }
}

fn deserialize_credit<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let credit: &str = Deserialize::deserialize(deserializer)?;

    let credit = credit.trim();

    if credit.is_empty() {
        return Ok(0.0);
    }

    let credit = credit.parse::<f64>().map_err(serde::de::Error::custom)?;

    Ok(credit)
}

pub fn read_csv(input_path: &str) -> Result<Vec<Record>, csv::Error> {
    let reader = csv::Reader::from_path(input_path)?;
    reader.into_deserialize().collect()
}
