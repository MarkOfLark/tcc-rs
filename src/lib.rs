
#![crate_name = "tcc"]
#![crate_type = "lib"]

extern crate time;
use std::default::Default;

pub static EPOCH: i64 = 864000;

pub struct Yearbase(pub Option<u64>);

pub struct Timestamp {
    ts: i64,
}

#[derive(Clone,Default)]
pub struct Calendar {
    year_sign: bool,
    delim_month: char,
    delim_day: char,
    delim_hour: char,
    delim_minute: char,
    delim_second: char,
    delim_desig: Option<char>,

    mod_quarter: i64,
    mod_month: i64,
    mod_week: i64,
    mod_day: i64,
    mod_hour: i64,
    mod_minute: i64,
    mod_second: i64,
}

pub struct Time {
    year: i64,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    nano_second: i32,
    year_base: Yearbase,
}

pub fn now() -> Timestamp {
    let utc = time::get_time();
    Timestamp {ts:utc.sec - EPOCH}
}

pub fn at(utc : time::Timespec) -> Timestamp {
    Timestamp {ts : utc.sec - EPOCH}
}


impl Calendar {
    pub fn new() -> Calendar {
        let cal: Calendar = Default::default();
        cal
    }

    pub fn at(&self, t : Time) -> String {
        t.to_string()
    }

    pub fn force_year_sign(&mut self, force : bool) -> Calendar {
        self.year_sign = force;
        (*self).clone()
    }

    pub fn month_delimiter(&mut self, delim: char) -> Calendar {
        self.delim_month = delim;
        (*self).clone()
    }

    pub fn designator_delimiter(&mut self, delim: Option<char>) -> Calendar {
        self.delim_desig = delim;
        (*self).clone()
    }
}

/* TODO see http://internals.rust-lang.org/t/orphan-rules/1322
impl collections::string::ToString for Calendar {
    pub fn to_string(&self) -> String {
        self.at(Time::at(time::Timespec{sec: EPOCH, nsec: 0},Yearbase(None)));
    }
}
*/



impl Time {
    fn new(utc : time::Timespec, year_base : Yearbase) -> Time {
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
        
        Time{year: year as i64,
             month: mon as i8,
             day: day as i8,
             hour: hour as i8,
             minute: min as i8,
             second: ts as i8,
             nano_second: utc.nsec as i32,
             year_base: year_base}
    }

    pub fn now(year_base : Yearbase) -> Time {
        let utc = time::get_time();
        Time::new(utc,year_base)
    }

    pub fn at (utc : time::Timespec, year_base : Yearbase) -> Time {
        Time::new(utc,year_base)
    }
}


impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"TC+{}",self.ts)
    }
}


impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let Yearbase(ybo) = self.year_base;
        let yb = match ybo {
            Some(value) => format!("TC{}",value),
            None        => "".to_string(),
        };

        let sec = self.second as f64 + 1e-9*(self.nano_second as f64);

        write!(f,"{}.{}.{},{}.{}.{} TC{}",
                 self.year,
                 self.month,
                 self.day,
                 self.hour,
                 self.minute,
                 sec,
                 yb)
    }
}

impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: insert formatting function here
        write!(f,"{}",self.at(Time::at(time::Timespec{sec: EPOCH, nsec: 0},Yearbase(None))))
    }
}
