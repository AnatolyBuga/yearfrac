use yearfrac;
use chrono::NaiveDate;

#[test]
fn test_accuracy() {
    use yearfrac::DayCountConvention;
    let delta = 1e-9;

    let start = NaiveDate::from_ymd(1978, 2, 28);
    let end = NaiveDate::from_ymd(2020, 5, 17);
    let yf = DayCountConvention::from_int(0).unwrap()
                    .yearfrac(start, end);
    assert!((yf - 42.21388888889).abs() < delta);
    let yf = DayCountConvention::from_int(1).unwrap()
                    .yearfrac(start, end);
    assert!((yf - 42.21424933147).abs() < delta);
    let yf = DayCountConvention::from_int(2).unwrap()
                    .yearfrac(start, end);
    assert!((yf - 42.83055555556).abs() < delta);
    let yf = DayCountConvention::from_int(3).unwrap()
                    .yearfrac(start, end);
    assert!((yf - 42.24383561644).abs() < delta);
    let yf = DayCountConvention::from_int(4).unwrap()
                    .yearfrac(start, end);
    assert!((yf - 42.21944444444).abs() < delta);

    let start = NaiveDate::from_ymd(1993, 12, 02);
    let end = NaiveDate::from_ymd(2022, 04, 18);
    let yf = DayCountConvention::from_str("nasd30/360").unwrap()
                    .yearfrac(start, end);
    assert!((yf - 28.37777777778).abs() < delta);
    let yf = DayCountConvention::from_str("act/act").unwrap()
                    .yearfrac(start, end);
    assert!((yf - 28.37638039609).abs() < delta);
    let yf = DayCountConvention::from_str("act360").unwrap()
                    .yearfrac(start, end);
    assert!((yf - 28.78888888889).abs() < delta);
    let yf = DayCountConvention::from_str("act365").unwrap()
                    .yearfrac(start, end);
    assert!((yf - 28.39452054795).abs() < delta);
    let yf = DayCountConvention::from_str("eur30/360").unwrap()
                    .yearfrac(start, end);
    assert!((yf - 28.37777777778).abs() < delta);
}

#[test]
#[should_panic]
fn test_bad_input () {
    use yearfrac::DayCountConvention;
    DayCountConvention::from_str("wrongvalue").unwrap();
}