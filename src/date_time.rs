use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use crate::errors::JourneyError;

pub struct DateTimeHandler {
    pub locale: String,
}

impl DateTimeHandler {
    pub fn new(locale: String) -> Self {
        Self { locale }
    }

    pub fn parse_date(&self, date_str: &str) -> Result<NaiveDate, JourneyError> {
        self.parse_date_with_format_override(date_str, None)
    }

    pub fn parse_date_with_format_override(&self, date_str: &str, format_override: Option<&str>) -> Result<NaiveDate, JourneyError> {
        // If format override is specified, use only that format
        if let Some(override_format) = format_override {
            // Convert common format names to chrono format strings
            let format_str = match override_format {
                "YYYY-MM-DD" => "%Y-%m-%d",
                "MM/DD/YYYY" => "%m/%d/%Y",
                "DD.MM.YYYY" => "%d.%m.%Y",
                "DD/MM/YYYY" => "%d/%m/%Y",
                "MM-DD-YYYY" => "%m-%d-%Y",
                "DD-MM-YYYY" => "%d-%m-%Y",
                _ => override_format, // Use as-is if it's already a chrono format
            };

            if let Ok(date) = NaiveDate::parse_from_str(date_str, format_str) {
                return Ok(date);
            }

            return Err(JourneyError::InvalidDateFormat(format!(
                "Could not parse date: {} with format override: {}",
                date_str, override_format
            )));
        }

        // Try different date formats based on locale (original behavior)
        let formats: Vec<&str> = if self.locale.starts_with("en") {
            // US/English formats (MM/DD/YYYY)
            vec![
                "%Y-%m-%d",     // ISO: 2025-10-24
                "%m/%d/%Y",     // US: 10/24/2025
                "%m-%d-%Y",     // US with dashes: 10-24-2025
                "%B %d, %Y",    // US long: October 24, 2025
                "%b %d, %Y",    // US short: Oct 24, 2025
            ]
        } else if self.locale.starts_with("no") || self.locale.starts_with("nb") || self.locale.starts_with("nn") {
            // Norwegian formats (DD.MM.YYYY)
            vec![
                "%Y-%m-%d",     // ISO: 2025-10-24
                "%d.%m.%Y",     // Norwegian: 24.10.2025
                "%d/%m/%Y",     // European: 24/10/2025
                "%d-%m-%Y",     // European with dashes: 24-10-2025
                "%d. %B %Y",    // Norwegian long: 24. oktober 2025
                "%d. %b %Y",    // Norwegian short: 24. okt 2025
            ]
        } else {
            // Default formats
            vec![
                "%Y-%m-%d",     // ISO: 2025-10-24
                "%m/%d/%Y",     // US: 10/24/2025
                "%d/%m/%Y",     // European: 24/10/2025
            ]
        };

        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }

        Err(JourneyError::InvalidDateFormat(format!(
            "Could not parse date: {} for locale: {}",
            date_str, self.locale
        )))
    }

    pub fn parse_time(&self, time_str: &str) -> Result<NaiveTime, JourneyError> {
        self.parse_time_with_format_override(time_str, None)
    }

    pub fn parse_time_with_format_override(&self, time_str: &str, format_override: Option<&str>) -> Result<NaiveTime, JourneyError> {
        // If format override is specified, use only that format
        if let Some(override_format) = format_override {
            let formats = match override_format {
                "12h" => vec![
                    "%I:%M %p",     // 12-hour: 2:30 PM
                    "%I:%M:%S %p",  // 12-hour with seconds: 2:30:45 PM
                    "%I:%M%p",      // 12-hour compact: 2:30PM
                    "%I:%M:%S%p",   // 12-hour compact with seconds: 2:30:45PM
                ],
                "24h" => vec![
                    "%H:%M",        // 24-hour: 14:30
                    "%H:%M:%S",     // 24-hour with seconds: 14:30:45
                ],
                _ => return Err(JourneyError::InvalidTimeFormat(format!(
                    "Invalid time format override: {}. Use '12h' or '24h'", override_format
                )))
            };

            for format in &formats {
                if let Ok(time) = NaiveTime::parse_from_str(time_str, format) {
                    return Ok(time);
                }
            }

            return Err(JourneyError::InvalidTimeFormat(format!(
                "Could not parse time: {} with format override: {}",
                time_str, override_format
            )));
        }

        // Try different time formats based on locale (original behavior)
        let formats: Vec<&str> = if self.locale.starts_with("en") {
            // US/English formats
            vec![
                "%H:%M",        // 24-hour: 14:30
                "%H:%M:%S",     // 24-hour with seconds: 14:30:45
                "%I:%M %p",     // 12-hour: 2:30 PM
                "%I:%M:%S %p",  // 12-hour with seconds: 2:30:45 PM
                "%I:%M%p",      // 12-hour compact: 2:30PM
                "%I:%M:%S%p",   // 12-hour compact with seconds: 2:30:45PM
            ]
        } else if self.locale.starts_with("no") || self.locale.starts_with("nb") || self.locale.starts_with("nn") {
            // Norwegian formats
            vec![
                "%H:%M",        // 24-hour: 14:30
                "%H:%M:%S",     // 24-hour with seconds: 14:30:45
                "%I:%M %p",     // 12-hour: 2:30 PM (English AM/PM)
                "%I:%M:%S %p",  // 12-hour with seconds: 2:30:45 PM
                "%I:%M%p",      // 12-hour compact: 2:30PM
                "%I:%M:%S%p",   // 12-hour compact with seconds: 2:30:45PM
            ]
        } else {
            // Default formats
            vec![
                "%H:%M",        // 24-hour: 14:30
                "%H:%M:%S",     // 24-hour with seconds: 14:30:45
                "%I:%M %p",     // 12-hour: 2:30 PM
                "%I:%M:%S %p",  // 12-hour with seconds: 2:30:45 PM
                "%I:%M%p",      // 12-hour compact: 2:30PM
                "%I:%M:%S%p",   // 12-hour compact with seconds: 2:30:45PM
            ]
        };

        for format in &formats {
            if let Ok(time) = NaiveTime::parse_from_str(time_str, format) {
                return Ok(time);
            }
        }

        Err(JourneyError::InvalidTimeFormat(format!(
            "Could not parse time: {} for locale: {}",
            time_str, self.locale
        )))
    }

    pub fn parse_relative_date(&self, days_ago: i64) -> NaiveDate {
        let now = Local::now().date_naive();
        now - chrono::Duration::days(days_ago)
    }

    pub fn format_date(&self, date: NaiveDate) -> String {
        date.format("%Y-%m-%d").to_string()
    }

    pub fn format_datetime(&self, datetime: DateTime<Local>) -> String {
        datetime.format("%H:%M:%S").to_string()
    }

    pub fn get_current_datetime(&self) -> DateTime<Local> {
        Local::now()
    }

    pub fn combine_date_time(&self, date: NaiveDate, time: NaiveTime) -> DateTime<Local> {
        let naive_dt = NaiveDateTime::new(date, time);
        Local.from_local_datetime(&naive_dt).single().unwrap_or_else(|| Local::now())
    }
}

