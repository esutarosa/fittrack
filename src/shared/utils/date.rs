use chrono::NaiveDate;

pub fn format_date(value: NaiveDate) -> String {
    value.format("%Y-%m-%d").to_string()
}
