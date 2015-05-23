extern crate time;
extern crate getopts;
extern crate tcc;

use time::{Timespec};
use getopts::Options;
use std::env;



fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]",program);
    print!("{}", opts.usage(&brief));
}


fn main() {
    // Get time right away so that printed time does not reflect
    // time spent parsing arguments and formatting
    let ts = tcc::timestamp_now();
    let t = tcc::Time::time_now(tcc::Yearbase(None));

    println!("The current timestamp: {}",ts);
    println!("The beginning timestamp: {}",tcc::timestamp_at(Timespec{sec: tcc::EPOCH, nsec: 0}));

    println!("The current time: {}",t);
    println!("The beginning time: {}",tcc::Time::time_at(Timespec{sec: tcc::EPOCH, nsec: 0},tcc::Yearbase(None)));

    // Get options from command line arguments in order to build Calendar
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("Y","year-base","Integer representing the year base","NUMBER");
    opts.optopt("Q","mod-quarter","Integer representing the datemod for quarters","NUMBER");
    opts.optopt("L","mod-luna","Integer representing the datemod for lunas","NUMBER");
    opts.optopt("W","mod-week","Integer representing the datemod for weeks","NUMBER");
    opts.optopt("D","mod-day","Integer representing the datemod for days","NUMBER");
    opts.optopt("R","mod-hour","Integer representing the datemod for hours","NUMBER");
    opts.optopt("M","mod-minute","Integer representing the datemod for minutes","NUMBER");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut c = tcc::Calendar::new().
                               force_year_sign(true).
                               month_delimiter('/').
                               designator_delimiter(None);


    println!("Calendar: {}",c);

}
