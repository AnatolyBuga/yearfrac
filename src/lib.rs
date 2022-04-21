use chrono::{NaiveDate, Datelike};
use std::{str::FromStr, error::Error, fmt::Display};
use thiserror::Error;

pub fn yearfrac(start: NaiveDate, end: NaiveDate, day_count_convention: DayCountConvention) -> f64 {
    let numerator = diff_dts(start, end, day_count_convention);
    let denom = basis(start, end, day_count_convention);
    numerator/denom
}

fn diff_dts(start: NaiveDate, end: NaiveDate, dcc: DayCountConvention) -> f64{
    match dcc { 
        DayCountConvention::ActAct |
        DayCountConvention::Act360 |
        DayCountConvention::Act365 
        => (start-end).num_days().abs() as f64,
        DayCountConvention::US30360 => {
            nasd360(start, end, 0, true)
        }
        DayCountConvention::EU30360=> {
            euro360(start, end)
        }
    }
}

fn euro360(start: NaiveDate, end: NaiveDate) -> f64 {
    let (mut start_day, start_month, start_year) = (start.day(), start.month(), start.year());
    let (mut end_day, end_month, end_year) = (end.day(), end.month(), end.year());
    if start_day == 31 {
        start_day = 30;
    };
    if end_day == 31 {
        end_day = 30;
    };
    days360(start_day, start_month, start_year, end_day, end_month, end_year)
}

fn nasd360 (start: NaiveDate, end: NaiveDate, method: u8, use_eom: bool) -> f64{
    let (mut start_day, start_month, start_year) = (start.day(), start.month(), start.year());
    let (mut end_day, end_month, end_year) = (end.day(), end.month(), end.year());
    if ((end_month == 2) & is_end_of_month(end_day, end_month, end_year)) &
    ( ((start_month == 2) & is_end_of_month(start_day, start_month, start_year)) | (method==3)) {
        end_day = 30;
    };
    if (end_day==31) & ( (start_day>=30) | (method==3)){
        end_day = 30;
    };
    if start_day==31 {
        start_day = 30;
    }
    if use_eom & (start_month==2) & is_end_of_month(start_day, start_month, start_year) {
        start_day = 30;
    }
    days360(start_day, start_month, start_year, end_day, end_month, end_year)
}

fn days360(start_day: u32, start_month: u32, start_year: i32, end_day: u32, end_month: u32, end_year: i32)->f64{
    ((end_year - start_year)*360 + (end_month as i32-start_month as i32)*30 + (end_day as i32-start_day as i32)).into()
}

fn basis(start: NaiveDate, end: NaiveDate, dcc: DayCountConvention) -> f64{
    match dcc {
        DayCountConvention::US30360 | 
        DayCountConvention::Act360 | 
        DayCountConvention::EU30360 
        => 360.0,
        DayCountConvention::Act365 => 
        365.0,
        DayCountConvention::ActAct => {
            let (start_day, start_month, start_year) = (start.day(), start.month(), start.year());
            let (end_day, end_month, end_year) = (end.day(), end.month(), end.year());
            if start_year == end_year {
                if is_leap_year(start_year) {
                    366.0
                } else {
                    365.0
                }
            } else if (end_year-1 == start_year) & 
              ( (start_month>end_month) | ( (start_month==end_month) & (start_day>end_day) ) )
                {
                    if is_leap_year(start_year) {
                        if (start_month < 2) | ( (start_month==2) & (start_day<=29)) {
                            366.0
                        } else {
                            365.0
                        }
                    } else if is_leap_year(end_year) {
                        if (end_month > 2) | ( (end_month==2) & (end_day==29)) {
                            366.0
                        }
                        else {
                            365.0
                        }
                    } else {
                    365.0
                    }
                }
            
            else{
                let mut tmp = 0.0;
                for i_y in start_year..end_year+1 {
                    if is_leap_year(i_y) {
                        tmp += 366.0
                    } else {
                        tmp += 365.0
                    }
                }
                tmp / (end_year as f64 - start_year as f64 + 1.0)
            }
        }
    }
}

pub fn is_leap_year(year: i32) -> bool {
    if year%4 > 0 {
        false
    } else if year%100 > 0 {
        true
    } else if year%400 == 0 {
        true
    }
    else {
        false
    }
}

pub fn is_end_of_month (day: u32, month: u32, year: i32) -> bool {
    if [1, 3, 5, 7, 8, 10, 12].contains(&month) {
        day == 31
    } else if [4, 6, 9, 11].contains(&month) {
        day == 30
    } else {
        if is_leap_year(year) {
            day == 29
        } else {
            day == 28
        }
    }
}


#[derive(Hash, Clone, Copy, Debug)] 
pub enum DayCountConvention {
    US30360,
    ActAct,
    Act360,
    Act365,
    EU30360
}

impl DayCountConvention {
    //type Err = DayCountConventionError;

    pub fn from_int(day_count_convention: u8) -> Result<Self, DayCountConventionError> {
        match day_count_convention {
            0 => Ok(DayCountConvention::US30360),
            1 => Ok(DayCountConvention::ActAct),
            2 => Ok(DayCountConvention::Act360),
            3 => Ok(DayCountConvention::Act365),
            4 => Ok(DayCountConvention::EU30360),
            other => Err(DayCountConventionError::InvalidValue{val: other.to_string()})
        }
    }
}

impl FromStr for DayCountConvention {
    type Err = DayCountConventionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "nasd30/360"     => Ok(DayCountConvention::US30360),
            "act/act"        => Ok(DayCountConvention::ActAct),
            "act360"         => Ok(DayCountConvention::Act360),
            "act365"         => Ok(DayCountConvention::Act365),
            "european30/360" => Ok(DayCountConvention::EU30360),
            other       => Err(DayCountConventionError::InvalidValue{val: other.to_owned()})
        }
    }
}

#[derive(Debug, Error)]
pub enum DayCountConventionError{
    #[error("Invalid Value: {}. Has to be one of: nasd30/360, act/act, act360, act365, european30/360 (from_str) 
    or in the range 0-4 (from_int)", val)]
    InvalidValue{val: String}
}
/*
impl Display for DayCountConventionError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match &*self {
            DayCountConventionError::InvalidValue{val} => {
                let tmp = "invalid value: ".to_owned() + val.clone().as_str();
                f.write_str(tmp.as_str())},
        }
    }
}
//impl Error for DayCountConventionError {}
// to use with ? operator
//impl From<ParseIntError> for DayCountConventionError {
//    fn from(err: ParseIntError) -> Self {
 //       DayCountConventionError::InvalidValue(err)
 //   }
//}
*/

#[cfg(test)]
mod tests {
    use crate::DayCountConvention;
    use crate::yearfrac;

    #[test]
    fn test_all() {
        use chrono::NaiveDate;
        let delta = 1e-9;

        let start = NaiveDate::from_ymd(1978, 2, 28);
        let end = NaiveDate::from_ymd(2020, 5, 17);
        let yf = yearfrac(start, end, DayCountConvention::from_int(0));
        assert!((yf - 42.21388888889).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(1));
        assert!((yf - 42.21424933147).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(2));
        assert!((yf - 42.83055555556).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(3));
        assert!((yf - 42.24383561644).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(4));
        assert!((yf - 42.21944444444).abs() < delta);

        let start = NaiveDate::from_ymd(1993, 12, 02);
        let end = NaiveDate::from_ymd(2022, 04, 18);
        let yf = yearfrac(start, end, DayCountConvention::from_int(0));
        assert!((yf - 28.37777777778).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(1));
        assert!((yf - 28.37638039609).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(2));
        assert!((yf - 28.78888888889).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(3));
        assert!((yf - 28.39452054795).abs() < delta);
        let yf = yearfrac(start, end, DayCountConvention::from_int(4));
        assert!((yf - 28.37777777778).abs() < delta);
    }
}
