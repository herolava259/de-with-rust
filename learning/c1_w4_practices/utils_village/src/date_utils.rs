//Develop a library that provides date and time manipulation functionalities. Include functions to calculate the difference between two dates, validate a date format, and format dates in different styles (e.g., DD/MM/YYYY, YYYY-MM-DD).
use chrono::NaiveDate;

pub enum FormatStyle {
    DDMMYYYY,
    YYYYMMDD,
}

impl FormatStyle {
    pub fn from_str(style: &str) -> Option<FormatStyle> {
        match style {
            "DD/MM/YYYY" => Some(FormatStyle::DDMMYYYY),
            "YYYY-MM-DD" => Some(FormatStyle::YYYYMMDD),
            _ => None,
        }
    }
}

pub fn calculate_date_difference(date1: &str, date2: &str) -> Result<i64, String> {
    let d1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let d2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d").map_err(|e| e.to_string())?;
    Ok((d2 - d1).num_days())
}

pub fn validate_date_format(date: &str) -> bool {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

pub fn format_date(date: &str, style: &str) -> Result<String, String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    match style {
        "DD/MM/YYYY" => Ok(d.format("%d/%m/%Y").to_string()),
        "YYYY-MM-DD" => Ok(d.format("%Y-%m-%d").to_string()),
        _ => Err("Unsupported date format style".to_string()),
    }
}


pub fn format_date_with_enum(date: &str, style: FormatStyle) -> Result<String, String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    match style {
        FormatStyle::DDMMYYYY => Ok(d.format("%d/%m/%Y").to_string()),
        FormatStyle::YYYYMMDD => Ok(d.format("%Y-%m-%d").to_string()),
    }
}



