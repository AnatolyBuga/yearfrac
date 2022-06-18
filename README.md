[yearfrac][docsrs]: Year Fruction for Rust
========================================
[cratesio]: https://crates.io/crates/yearfrac
[docsrs]: https://docs.rs/yearfrac/

It was tested to match Excel's YEARFRAC function and with time go beyond. We support all 5 Excel's methodologies:
nasd360
act/act
act360
act365
eur360

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
yearfrac = "0.1"
```

## Examples

 ```rust
 use yearfrac::DayCountConvention;
 use chrono::{NaiveDate, Datelike};
 let start = NaiveDate::from_ymd(1978, 2, 28);
 let end = NaiveDate::from_ymd(2020, 5, 17);
 let yf = DayCountConvention::from_int(0).unwrap()
                .yearfrac(start, end);
assert!((yf - 42.21388888889).abs() < 1e-9);
 
 let yf = DayCountConvention::from_str("act/act").unwrap()
             .yearfrac(start, end);
 assert!((yf - 42.21424933147).abs() < 1e-9);

 use yearfrac::is_leap_year;
 assert_eq!(is_leap_year(start.year()) as i32, 0)

 let yf = DayCountConvention::US30360.yearfrac_signed(end, start);
 assert!((yf + 42.21388888889).abs() < 1e-9);
 ```
