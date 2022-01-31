extern crate getopts;
use getopts::Options;
use std::env;

mod judgement;
mod read_csv;

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

    let mut records = read_csv::read_csv(input_path.as_str()).expect("Failed to read csv");

    println!("{:?}", records);
}
