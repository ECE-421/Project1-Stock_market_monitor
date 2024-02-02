use yahoo_finance_api as yahoo;
use chrono::{DateTime, Utc};
use chronoutil::delta;
use chrono_tz::{Tz, UTC};
use time::{OffsetDateTime};
use std::time::SystemTime;


fn date_converter(start_date: DateTime<Utc>, shift_months: i32) -> DateTime<Utc> {
    let start_date = delta::shift_months(start_date, shift_months);
    // maybe convert time to start of day??
    
    return OffsetDateTime::from(system_time);
}

fn historic_company_data(ticker: &str, start_date: DateTime<Utc>, shift_months: i32) {
    let provider = yahoo::YahooConnector::new();

    let end_date = date_converter(start_date, -shift_months);

    let historic_data = tokio_test::block_on(provider.get_quote_history(ticker, start_date, end_date)).unwrap();
}

fn main() {
    let provider = yahoo::YahooConnector::new();

    // Current date and time in UTC format
    let current_datetime = Utc::now();
    
    
    // Example: Shift the date 6 months back
    let start_date = date_converter(current_datetime, -6);

    println!("Start Date: {}", start_date);
    println!("End Date: {}", current_datetime);
    
}
