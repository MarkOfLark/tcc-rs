
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
/// Object for formatting TCC timestamps
pub struct Calendar {
    /// Indicates if the sign (+/-) should always be displayed before the year feild.
    year_sign: bool,
    /// Delimiter that comes between the year and month
    delim_month: char,
    /// Delimiter that comes between the month and day
    delim_day: char,
    /// Delimiter that comes between the day and hour
    delim_hour: char,
    /// Delimiter that comes between the hour and minute
    delim_minute: char,
    /// Delimiter that comes between the minute and second
    delim_second: char,
    /// Delimiter that comes between the second and year base designator
    delim_desig: Option<char>,

    /// fields to hold the date mod
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

pub fn timestamp_now() -> Timestamp {
    let utc = time::get_time();
    Timestamp {ts:utc.sec - EPOCH}
}

pub fn timestamp_at(utc : time::Timespec) -> Timestamp {
    Timestamp {ts : utc.sec - EPOCH}
}

fn acceptable_delimiter(delim: char) -> bool {
    match delim {
        ','|'/'|' '|'_'|':'|'.' => true,
                              _ => false,
    } 
}

impl Calendar {
    pub fn new() -> Calendar {
        let cal: Calendar = Default::default();
        cal
    }

    pub fn calendar_at(&self, t : Time) -> String {
        // TODO apply formatting
        t.to_string()
    }

    pub fn force_year_sign(&mut self, force : bool) -> Calendar {
        self.year_sign = force;
        (*self).clone()
    }

    pub fn month_delimiter(&mut self, delim: char) -> Calendar {
        if acceptable_delimiter(delim) {
            self.delim_month = delim;
        }
        (*self).clone()
    }

    pub fn day_delimiter(&mut self, delim: char) -> Calendar {
        if acceptable_delimiter(delim) {
            self.delim_day = delim;
        }
        (*self).clone()
    }

    pub fn hour_delimiter(&mut self, delim: char) -> Calendar {
        if acceptable_delimiter(delim) {
            self.delim_hour = delim;
        }
        (*self).clone()
    }

    pub fn minute_delimiter(&mut self, delim: char) -> Calendar {
        if acceptable_delimiter(delim) {
            self.delim_minute = delim;
        }
        (*self).clone()
    }

    pub fn second_delimiter(&mut self, delim: char) -> Calendar {
        if acceptable_delimiter(delim) {
            self.delim_second = delim;
        }
        (*self).clone()
    }

    pub fn designator_delimiter(&mut self, delim: Option<char>) -> Calendar {
        self.delim_desig = match delim {
            Some(d) => match acceptable_delimiter(d) {
                true => delim,
                false => self.delim_desig,
            },
            None => delim,
        };
        (*self).clone()
    }

}



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
       
        // TODO factor in leaps based on yearbase

        Time{year: year as i64,
             month: mon as i8,
             day: day as i8,
             hour: hour as i8,
             minute: min as i8,
             second: ts as i8,
             nano_second: utc.nsec as i32,
             year_base: year_base}
    }

    pub fn time_now(year_base : Yearbase) -> Time {
        let utc = time::get_time();
        Time::new(utc,year_base)
    }

    pub fn time_at (utc : time::Timespec, year_base : Yearbase) -> Time {
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
        write!(f,"{}",self.calendar_at(Time::time_at(time::Timespec{sec: EPOCH, nsec: 0},Yearbase(None))))
    }
}
