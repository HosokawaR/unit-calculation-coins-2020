extern crate getopts;
use getopts::Options;
use std::env;

mod display;
mod judgement;
mod read_csv;
mod read_toml;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} CSV_PATH [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "HELP HELP HELP");
    opts.optflag("p", "prospect", "include course you are taking now.");

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

    let input_path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut requirements = read_toml::read_toml("coins-2020.toml");

    let mut records = read_csv::read_csv(input_path.as_str()).expect("Failed to read csv");

    judgement::judge(&mut requirements, &mut records, &matches.opt_present("p"));

    display::display_result(&requirements, "", "");
}
