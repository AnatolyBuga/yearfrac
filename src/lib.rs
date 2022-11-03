//! # yearfrac
//! `yearfrac` is a rust crate used to calculate fraction of a year between two dates(chrono::NaiveDate).
//!
//! Supports all five Excel's daycount conventions:
//!
//! nasd30/360
//!
//! act/act
//!
//! act360  
//!   
//! act365
//!    
//! eur30/360
//!
//! Tested to match Excel's YEARFRAC function
//! # Examples
//! ```rust
//! use yearfrac::DayCountConvention;
//! use chrono::{NaiveDate, Datelike};
//! let start = NaiveDate::from_ymd(1978, 2, 28);
//! let end = NaiveDate::from_ymd(2020, 5, 17);
//! let yf = DayCountConvention::from_int(0).unwrap()
//!                .yearfrac(start, end);
//!assert!((yf - 42.21388888889).abs() < 1e-9);
//!
//! let yf = DayCountConvention::from_str("act/act").unwrap()
//!             .yearfrac(start, end);
//! assert!((yf - 42.21424933147).abs() < 1e-9);
//!
//! use yearfrac::is_leap_year;
//! assert_eq!(is_leap_year(start.year()) as i32, 0);
//!
//! let yf = DayCountConvention::US30360.yearfrac_signed(end, start);
//! assert!((yf + 42.21388888889).abs() < 1e-9);
//! ```

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// #Examples
/// ```rust
/// use chrono::{NaiveDate, Datelike};
/// let dt = NaiveDate::from_ymd(1978, 2, 28);
///
/// use yearfrac::is_leap_year;
/// assert_eq!(is_leap_year(dt.year()) as i32, 0)
#[allow(clippy::if_same_then_else)]
pub fn is_leap_year(year: i32) -> bool {
    if year % 4 > 0 {
        false
    } else if year % 100 > 0 {
        true
    } else if year % 400 == 0 {
        true
    } else {
        false
    }
}

/// #Examples
/// ```rust
/// use chrono::{NaiveDate, Datelike};
/// let dt = NaiveDate::from_ymd(1978, 2, 28);
///
/// use yearfrac::is_end_of_month;
/// assert!(is_end_of_month(dt.day(), dt.month(), dt.year()))
pub fn is_end_of_month(day: u32, month: u32, year: i32) -> bool {
    if [1, 3, 5, 7, 8, 10, 12].contains(&month) {
        day == 31
    } else if [4, 6, 9, 11].contains(&month) {
        day == 30
    } else if is_leap_year(year) {
        day == 29
    } else {
        day == 28
    }
}

#[derive(Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DayCountConvention {
    US30360,
    ActAct,
    Act360,
    Act365,
    EU30360,
}

impl DayCountConvention {
    /// Generates DayCountConvention enum from an u8;
    /// Acceptable values:
    ///
    /// 0 for nasd30/360
    ///
    /// 1 for act/act
    ///
    /// 2 for act360  
    ///   
    /// 3 for act365
    ///    
    /// 4 for eur30/360
    ///
    /// # Examples
    /// ```rust
    /// use yearfrac::DayCountConvention;
    /// let yf = DayCountConvention::from_int(3).unwrap();
    /// ```
    /// # Panics
    ///  ```should_panic
    /// use yearfrac::DayCountConvention;
    ///
    /// let yf = DayCountConvention::from_int(5).unwrap();
    /// ```
    pub fn from_int(day_count_convention: u8) -> Result<Self, DayCountConventionError> {
        match day_count_convention {
            0 => Ok(DayCountConvention::US30360),
            1 => Ok(DayCountConvention::ActAct),
            2 => Ok(DayCountConvention::Act360),
            3 => Ok(DayCountConvention::Act365),
            4 => Ok(DayCountConvention::EU30360),
            other => Err(DayCountConventionError::InvalidValue {
                val: other.to_string(),
            }),
        }
    }
    /// Generates DayCountConvention enum from a &str;
    /// Acceptable values:
    ///
    /// nasd30/360
    ///
    /// act/act
    ///
    /// act360  
    ///   
    /// act365
    ///    
    /// eur30/360
    ///
    /// /// # Examples
    /// ```rust
    /// use yearfrac::DayCountConvention;
    /// let yf = DayCountConvention::from_str("act/act").unwrap();
    /// ```
    ///
    /// # Panics
    ///  ```should_panic
    /// use yearfrac::DayCountConvention;
    /// use yearfrac::DayCountConventionError;
    /// let yf = DayCountConvention::from_str("invalid str")?;
    /// # Ok::<(), DayCountConventionError>(())
    /// ```
    pub fn from_str(day_count_convention: &str) -> Result<Self, DayCountConventionError> {
        <Self as FromStr>::from_str(day_count_convention)
    }

    /// Calculates year fruction.
    /// # Examples
    /// ```rust
    /// use yearfrac::DayCountConvention;
    /// use chrono::NaiveDate;
    /// let start = NaiveDate::from_ymd(1978, 2, 28);
    /// let end = NaiveDate::from_ymd(2020, 5, 17);
    /// let yf = DayCountConvention::from_int(0).unwrap()
    ///                .yearfrac(start, end);
    ///assert!((yf - 42.21388888889).abs() < 1e-9);
    /// ```
    pub fn yearfrac(&self, mut start: NaiveDate, mut end: NaiveDate) -> f64 {
        if start == end {
            return 0.0; //edge case
        } else if start > end {
            (start, end) = (end, start)
        }
        let numerator = self.diff_dts(start, end);
        let denom = self.basis(start, end);
        numerator / denom
    }
    /// Signed version of yearfrac function.
    /// Returns negative value if start > end
    /// # Examples
    /// ```rust
    /// use yearfrac::DayCountConvention;
    /// use chrono::NaiveDate;
    /// let end = NaiveDate::from_ymd(1978, 2, 28);
    /// let start = NaiveDate::from_ymd(2020, 5, 17);
    /// let yf = DayCountConvention::US30360.yearfrac_signed(start, end);
    ///assert!((yf + 42.21388888889).abs() < 1e-9);
    /// ```
    pub fn yearfrac_signed(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        if start > end {
            -self.yearfrac(start, end)
        } else {
            self.yearfrac(start, end)
        }
    }

    fn basis(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        match self {
            DayCountConvention::US30360
            | DayCountConvention::Act360
            | DayCountConvention::EU30360 => 360.0,
            DayCountConvention::Act365 => 365.0,
            DayCountConvention::ActAct => {
                let (start_day, start_month, start_year) =
                    (start.day(), start.month(), start.year());
                let (end_day, end_month, end_year) = (end.day(), end.month(), end.year());
                if start_year == end_year {
                    if is_leap_year(start_year) {
                        366.0
                    } else {
                        365.0
                    }
                } else if (end_year - 1 == start_year)
                    & ((start_month > end_month)
                        | ((start_month == end_month) & (start_day > end_day)))
                {
                    if is_leap_year(start_year) {
                        if (start_month < 2) | ((start_month == 2) & (start_day <= 29)) {
                            366.0
                        } else {
                            365.0
                        }
                    } else if is_leap_year(end_year) {
                        if (end_month > 2) | ((end_month == 2) & (end_day == 29)) {
                            366.0
                        } else {
                            365.0
                        }
                    } else {
                        365.0
                    }
                } else {
                    let mut tmp = 0.0;
                    for i_y in start_year..end_year + 1 {
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

    fn diff_dts(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        match self {
            DayCountConvention::ActAct
            | DayCountConvention::Act360
            | DayCountConvention::Act365 => (end - start).num_days() as f64,
            DayCountConvention::US30360 => self.nasd360(start, end, 0, true),
            DayCountConvention::EU30360 => self.euro360(start, end),
        }
    }

    fn euro360(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        let (mut start_day, start_month, start_year) = (start.day(), start.month(), start.year());
        let (mut end_day, end_month, end_year) = (end.day(), end.month(), end.year());
        if start_day == 31 {
            start_day = 30;
        };
        if end_day == 31 {
            end_day = 30;
        };
        self.days360(
            start_day,
            start_month,
            start_year,
            end_day,
            end_month,
            end_year,
        )
    }

    /// NASD360 Needs work on methods (currently only Excel's third method)
    fn nasd360(&self, start: NaiveDate, end: NaiveDate, method: u8, use_eom: bool) -> f64 {
        let (mut start_day, start_month, start_year) = (start.day(), start.month(), start.year());
        let (mut end_day, end_month, end_year) = (end.day(), end.month(), end.year());
        if ((end_month == 2) & is_end_of_month(end_day, end_month, end_year))
            & (((start_month == 2) & is_end_of_month(start_day, start_month, start_year))
                | (method == 3))
        {
            end_day = 30;
        };
        if (end_day == 31) & ((start_day >= 30) | (method == 3)) {
            end_day = 30;
        };
        if start_day == 31 {
            start_day = 30;
        }
        if use_eom & (start_month == 2) & is_end_of_month(start_day, start_month, start_year) {
            start_day = 30;
        }
        self.days360(
            start_day,
            start_month,
            start_year,
            end_day,
            end_month,
            end_year,
        )
    }

    fn days360(
        &self,
        start_day: u32,
        start_month: u32,
        start_year: i32,
        end_day: u32,
        end_month: u32,
        end_year: i32,
    ) -> f64 {
        ((end_year - start_year) * 360
            + (end_month as i32 - start_month as i32) * 30
            + (end_day as i32 - start_day as i32))
            .into()
    }
}

impl FromStr for DayCountConvention {
    type Err = DayCountConventionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nasd30/360" => Ok(DayCountConvention::US30360),
            "act/act" => Ok(DayCountConvention::ActAct),
            "act360" => Ok(DayCountConvention::Act360),
            "act365" => Ok(DayCountConvention::Act365),
            "eur30/360" => Ok(DayCountConvention::EU30360),
            other => Err(DayCountConventionError::InvalidValue {
                val: other.to_owned(),
            }),
        }
    }
}

#[derive(Debug, Error)]
pub enum DayCountConventionError {
    #[error("Yearfrac: Invalid Value: {}. Has to be one of: nasd30/360, act/act, act360, act365, eur30/360 (from_str) 
    or in the range 0-4 (from_int).", val)]
    InvalidValue { val: String },
}

#[cfg(test)]
mod tests {}
