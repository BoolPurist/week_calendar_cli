use chrono::{Datelike, Local, NaiveDate};
use date_validation_types::units::ValidatedYear;

pub fn get_today_date() -> NaiveDate {
    Local::now().naive_local().date()
}
pub fn get_current_year() -> ValidatedYear {
    let year = get_today_date().year() as u32;
    ValidatedYear::new(year).unwrap()
}
