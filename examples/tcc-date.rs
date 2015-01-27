extern crate tcc;
extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,usage};

fn main() {
    // Get time right away so that printed time does not reflect
    // time spent parsing arguments and formatting
    let t = tcc::now();

    // Get options from command line arguments
    let opts = &[
        optflag("h", "help", "print this help menu"),
        optopt("y","year-base","Integer representing the year base","NUMBER"),
        optopt("Q","mod-quarter","Integer representing the datemod for quarters","NUMBER"),
        optopt("L","mod-luna","Integer representing the datemod for lunas","NUMBER"),
        optopt("W","mod-week","Integer representing the datemod for weeks","NUMBER"),
        optopt("D","mod-day","Integer representing the datemod for days","NUMBER"),
        optopt("H","mod-hour","Integer representing the datemod for hours","NUMBER"),
        optopt("M","mod-minute","Integer representing the datemod for minutes","NUMBER")
    ];
   

    let matches = match getopts(std::os::args().tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print!("{}", usage("tcc-date", opts));
        println!("\nExample: tcc-date -h 5");
        return;
    }



    println!("{}",t);

}
