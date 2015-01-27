extern crate time;
extern crate getopts;
extern crate tcc;

use std::os::args;
use time::{Timespec};
use getopts::{optopt,optflag,getopts,OptGroup,usage};



fn main() {
    // Get time right away so that printed time does not reflect
    // time spent parsing arguments and formatting
    let ts = tcc::now();
    let t = tcc::Time::now(tcc::Yearbase(None));

    // Get options from command line arguments
    let opts = &[
        optflag("h", "help", "print this help menu"),
        optopt("Y","year-base","Integer representing the year base","NUMBER"),
        optopt("Q","mod-quarter","Integer representing the datemod for quarters","NUMBER"),
        optopt("L","mod-luna","Integer representing the datemod for lunas","NUMBER"),
        optopt("W","mod-week","Integer representing the datemod for weeks","NUMBER"),
        optopt("D","mod-day","Integer representing the datemod for days","NUMBER"),
        optopt("R","mod-hour","Integer representing the datemod for hours","NUMBER"),
        optopt("M","mod-minute","Integer representing the datemod for minutes","NUMBER")
    ];
   

    let matches = match getopts(args().tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print!("{}", usage("tcc-date", opts));
        println!("\nExample: tcc-date -h 5");
        return;
    }

    println!("The current timestamp: {}",ts);
    println!("The beginning timestamp: {}",tcc::at(Timespec{sec: tcc::EPOCH, nsec: 0}));

    println!("The current time: {}",t);
    println!("The beginning time: {}",tcc::Time::at(Timespec{sec: tcc::EPOCH, nsec: 0},tcc::Yearbase(None)));
}
