
#![crate_name = "tcc"]
#![crate_type = "lib"]

extern crate time;

#[test]
fn it_works() {
}

pub struct Timestamp {
    ts: i64,
}

pub struct TCDate {
    year: i64,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    nano_second: i32,
    year_base: u64,
    mod_quarter: i64,
    mod_month: i64,
    mod_week: i64,
    mod_day: i64,
    mod_hour: i64,
    mod_minute: i64,
    mod_second: i64,
}

pub fn now() -> Timestamp{
    let utc = time::get_time();
    Timestamp{ts:utc.sec - 864000}
}

    /*
    let mut ts : i64 = utc.sec -864000;
    let year = ts / (60*60*24*28*13);
    ts = ts - year*13*28*24*60*60;
    let mon = ts / (60*60*24*28);
    ts = ts - mon*28*24*60*60;
    let day = ts / (60*60*24);
    ts = ts - day*24*60*60;
    let hour = ts / (60*60);
    ts = ts - hour*60*60;
    let min = ts / 60;
    ts = ts - min*60;

    TCDate{year: year as i64,
           month: mon as i8,
           day: day as i8,
           hour: hour as i8,
           minute: min as i8,
           second: ts as i8,
           nano_second: utc.nsec as i32,
           year_base: 0u64,
           mod_quarter: 0i64,
           mod_month: 0i64,
           mod_week: 0i64,
           mod_day: 0i64,
           mod_hour: 0i64,
           mod_minute: 0i64,
           mod_second: 0i64}
           */

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"TC+{}",self.ts)
    }
}
