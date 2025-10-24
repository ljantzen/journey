use journey::date_time::DateTimeHandler;
use chrono::{NaiveDate, NaiveTime, Local};

#[test]
fn test_date_handler_creation() {
    let handler = DateTimeHandler::new("en-US".to_string());
    assert_eq!(handler.locale, "en-US");
}

// ============================================================================
// US LOCALE TESTS
// ============================================================================

#[test]
fn test_us_locale_date_parsing() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    // Test various US date formats
    let test_cases = [
        ("2025-10-24", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
        ("10/24/2025", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
        ("10-24-2025", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
    ];
    
    for (date_str, expected) in test_cases {
        let result = handler.parse_date(date_str).unwrap();
        assert_eq!(result, expected, "Failed to parse US date: {}", date_str);
    }
}

#[test]
fn test_us_locale_time_parsing_24h() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    // Test 24-hour format
    let test_cases = [
        ("14:30", NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
        ("14:30:45", NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
        ("09:15", NaiveTime::from_hms_opt(9, 15, 0).unwrap()),
        ("23:59", NaiveTime::from_hms_opt(23, 59, 0).unwrap()),
    ];
    
    for (time_str, expected) in test_cases {
        let result = handler.parse_time(time_str).unwrap();
        assert_eq!(result, expected, "Failed to parse US 24h time: {}", time_str);
    }
}

#[test]
fn test_us_locale_time_parsing_12h() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    // Test 12-hour format
    let test_cases = [
        ("2:30 PM", NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
        ("2:30:45 PM", NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
        ("9:15 AM", NaiveTime::from_hms_opt(9, 15, 0).unwrap()),
        ("11:59 PM", NaiveTime::from_hms_opt(23, 59, 0).unwrap()),
        ("12:00 PM", NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
        ("12:00 AM", NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        ("2:30PM", NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
        ("2:30:45PM", NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
    ];
    
    for (time_str, expected) in test_cases {
        let result = handler.parse_time(time_str).unwrap();
        assert_eq!(result, expected, "Failed to parse US 12h time: {}", time_str);
    }
}

#[test]
fn test_us_locale_invalid_time() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    let invalid_times = ["25:00", "13:00 PM", "invalid", "25:30", "14:70"];
    
    for time_str in invalid_times {
        let result = handler.parse_time(time_str);
        assert!(result.is_err(), "Should fail to parse invalid US time: {}", time_str);
    }
}

// ============================================================================
// NORWEGIAN LOCALE TESTS
// ============================================================================

#[test]
fn test_norwegian_locale_date_parsing() {
    let handler = DateTimeHandler::new("no-NO".to_string());
    
    // Test various Norwegian date formats
    let test_cases = [
        ("2025-10-24", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
        ("24.10.2025", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
        ("24/10/2025", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
        ("24-10-2025", NaiveDate::from_ymd_opt(2025, 10, 24).unwrap()),
    ];
    
    for (date_str, expected) in test_cases {
        let result = handler.parse_date(date_str).unwrap();
        assert_eq!(result, expected, "Failed to parse Norwegian date: {}", date_str);
    }
}

#[test]
fn test_norwegian_locale_time_parsing_24h() {
    let handler = DateTimeHandler::new("no-NO".to_string());
    
    // Test 24-hour format (same as US)
    let test_cases = [
        ("14:30", NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
        ("14:30:45", NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
        ("09:15", NaiveTime::from_hms_opt(9, 15, 0).unwrap()),
        ("23:59", NaiveTime::from_hms_opt(23, 59, 0).unwrap()),
    ];
    
    for (time_str, expected) in test_cases {
        let result = handler.parse_time(time_str).unwrap();
        assert_eq!(result, expected, "Failed to parse Norwegian 24h time: {}", time_str);
    }
}

#[test]
fn test_norwegian_locale_time_parsing_12h() {
    let handler = DateTimeHandler::new("no-NO".to_string());
    
    // Test 12-hour format (using English AM/PM)
    let test_cases = [
        ("2:30 PM", NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
        ("2:30:45 PM", NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
        ("9:15 AM", NaiveTime::from_hms_opt(9, 15, 0).unwrap()),
        ("11:59 PM", NaiveTime::from_hms_opt(23, 59, 0).unwrap()),
        ("2:30PM", NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
        ("2:30:45PM", NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
    ];
    
    for (time_str, expected) in test_cases {
        let result = handler.parse_time(time_str).unwrap();
        assert_eq!(result, expected, "Failed to parse Norwegian 12h time: {}", time_str);
    }
}

#[test]
fn test_norwegian_locale_invalid_time() {
    let handler = DateTimeHandler::new("no-NO".to_string());
    
    let invalid_times = ["25:00", "13:00 PM", "invalid", "25:30", "14:70"];
    
    for time_str in invalid_times {
        let result = handler.parse_time(time_str);
        assert!(result.is_err(), "Should fail to parse invalid Norwegian time: {}", time_str);
    }
}

// ============================================================================
// CROSS-LOCALE COMPATIBILITY TESTS
// ============================================================================

#[test]
fn test_iso_date_cross_locale() {
    let us_handler = DateTimeHandler::new("en-US".to_string());
    let no_handler = DateTimeHandler::new("no-NO".to_string());
    
    let iso_date = "2025-10-24";
    let expected = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    
    let us_result = us_handler.parse_date(iso_date).unwrap();
    let no_result = no_handler.parse_date(iso_date).unwrap();
    
    assert_eq!(us_result, expected);
    assert_eq!(no_result, expected);
    assert_eq!(us_result, no_result);
}

#[test]
fn test_24h_time_cross_locale() {
    let us_handler = DateTimeHandler::new("en-US".to_string());
    let no_handler = DateTimeHandler::new("no-NO".to_string());
    
    let time_24h = "14:30:45";
    let expected = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
    
    let us_result = us_handler.parse_time(time_24h).unwrap();
    let no_result = no_handler.parse_time(time_24h).unwrap();
    
    assert_eq!(us_result, expected);
    assert_eq!(no_result, expected);
    assert_eq!(us_result, no_result);
}

#[test]
fn test_12h_time_cross_locale() {
    let us_handler = DateTimeHandler::new("en-US".to_string());
    let no_handler = DateTimeHandler::new("no-NO".to_string());
    
    let time_12h = "2:30:45 PM";
    let expected = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
    
    let us_result = us_handler.parse_time(time_12h).unwrap();
    let no_result = no_handler.parse_time(time_12h).unwrap();
    
    assert_eq!(us_result, expected);
    assert_eq!(no_result, expected);
    assert_eq!(us_result, no_result);
}

// ============================================================================
// LOCALE-SPECIFIC DATE FORMAT TESTS
// ============================================================================

#[test]
fn test_us_specific_date_formats() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    // Test US-specific formats that should work
    let us_formats = [
        "10/24/2025",  // MM/DD/YYYY
        "10-24-2025",  // MM-DD-YYYY
    ];
    
    for date_str in us_formats {
        let result = handler.parse_date(date_str);
        assert!(result.is_ok(), "US handler should parse US format: {}", date_str);
        assert_eq!(result.unwrap(), NaiveDate::from_ymd_opt(2025, 10, 24).unwrap());
    }
}

#[test]
fn test_norwegian_specific_date_formats() {
    let handler = DateTimeHandler::new("no-NO".to_string());
    
    // Test Norwegian-specific formats that should work
    let no_formats = [
        "24.10.2025",  // DD.MM.YYYY
        "24/10/2025",  // DD/MM/YYYY
        "24-10-2025",  // DD-MM-YYYY
    ];
    
    for date_str in no_formats {
        let result = handler.parse_date(date_str);
        assert!(result.is_ok(), "Norwegian handler should parse Norwegian format: {}", date_str);
        assert_eq!(result.unwrap(), NaiveDate::from_ymd_opt(2025, 10, 24).unwrap());
    }
}

#[test]
fn test_cross_locale_date_ambiguity() {
    let us_handler = DateTimeHandler::new("en-US".to_string());
    let no_handler = DateTimeHandler::new("no-NO".to_string());
    
    // Test ambiguous date that means different things in different locales
    let ambiguous_date = "01/02/2025"; // Could be Jan 2 or Feb 1
    
    let us_result = us_handler.parse_date(ambiguous_date).unwrap();
    let no_result = no_handler.parse_date(ambiguous_date).unwrap();
    
    // US should interpret as MM/DD/YYYY (Jan 2)
    assert_eq!(us_result, NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());
    // Norwegian should interpret as DD/MM/YYYY (Feb 1)
    assert_eq!(no_result, NaiveDate::from_ymd_opt(2025, 2, 1).unwrap());
    
    // They should be different
    assert_ne!(us_result, no_result);
}

// ============================================================================
// LEGACY TESTS (keeping for backward compatibility)
// ============================================================================

#[test]
fn test_parse_date_invalid_format() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    let result = handler.parse_date("invalid-date");
    assert!(result.is_err());
}

#[test]
fn test_parse_time_invalid_format() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    let result = handler.parse_time("invalid-time");
    assert!(result.is_err());
}

#[test]
fn test_parse_relative_date() {
    let handler = DateTimeHandler::new("en-US".to_string());
    
    // Test 0 days ago (today)
    let date = handler.parse_relative_date(0);
    let today = Local::now().date_naive();
    assert_eq!(date, today);
    
    // Test 1 day ago (yesterday)
    let date = handler.parse_relative_date(1);
    let yesterday = today - chrono::Duration::days(1);
    assert_eq!(date, yesterday);
}

#[test]
fn test_format_date() {
    let handler = DateTimeHandler::new("en-US".to_string());
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let formatted = handler.format_date(date);
    assert_eq!(formatted, "2025-10-24");
}

#[test]
fn test_format_datetime() {
    let handler = DateTimeHandler::new("en-US".to_string());
    let datetime = Local::now();
    let formatted = handler.format_datetime(datetime);
    // Should only contain time (HH:MM:SS format)
    assert!(formatted.contains(":"));
    // Should not contain year (no date)
    assert!(!formatted.contains("2025"));
    // Should be in HH:MM:SS format
    assert!(formatted.matches(':').count() == 2);
}

#[test]
fn test_combine_date_time() {
    let handler = DateTimeHandler::new("en-US".to_string());
    let date = NaiveDate::from_ymd_opt(2025, 10, 24).unwrap();
    let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    
    let combined = handler.combine_date_time(date, time);
    assert_eq!(combined.date_naive(), date);
    assert_eq!(combined.time(), time);
}
