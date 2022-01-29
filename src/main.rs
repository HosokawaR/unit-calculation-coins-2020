extern crate getopts;
use getopts::Options;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "学籍番号")]
    student_number: String,
    #[serde(rename = "学生氏名")]
    student_name: String,
    #[serde(rename = "科目番号")]
    course_code: String,
    #[serde(rename = "科目名")]
    course_name: String,
    // csv の単位数カラムに余分な空白があり serde の標準機能では f64 に変換できないため独自の変換関数を用意
    #[serde(rename = "単位数", deserialize_with = "deserialize_credit")]
    credit: f64,
    #[serde(rename = "春学期")]
    spring: String,
    #[serde(rename = "秋学期")]
    fall: String,
    #[serde(rename = "総合評価")]
    overall_grade: String,
    #[serde(rename = "科目区分")]
    course_category: String,
    #[serde(rename = "開講年度")]
    semester_year: u32,
    #[serde(rename = "開講区分")]
    semester_status: String,
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

fn calc(input_path: &str, output_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(input_path)?;

    for result in reader.deserialize() {
        let record: Record = result.expect("Reading CSV failed");
        println!("{:?}", record);
    }

    match output_path {
        Some(path) => {
            println!("Writing CSV to {}", path);
        }
        None => {}
    }

    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} CSV_PATH [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "HELP HELP HELP");
    opts.optflag("o", "output", "output path");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f)
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output_path = matches.opt_str("o");

    let input_path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    calc(&input_path, output_path);
}
