mod plotting;
mod utils;
mod command_line_parser; 
use yahoo_finance_api as yahoo;
use chrono::{DateTime, Utc};
use chronoutil::delta;
use chrono_tz::{Tz, UTC};
use time::{OffsetDateTime};
use std::time::SystemTime;


fn date_converter(start_date: DateTime<Utc>, shift_months: i32) -> DateTime<Utc> {
    let start_date = delta::shift_months(start_date, shift_months);
    // maybe convert time to start of day??
    
    return DateTime::from(start_date);
}

fn main() {
    let provider = yahoo::YahooConnector::new();

    // Current date and time in UTC format
    let current_datetime = Utc::now();
    
    // Example: Shift the date 6 months back
    let start_date = date_converter(current_datetime, -6);

    println!("Start Date: {}", start_date);
    println!("End Date: {}", current_datetime);

    let stock_ticker = command_line_parser::read_stock_ticker();
    if command_line_parser::check_stock_ticker(stock_ticker) {
        // api
        // plot
    }
    
}
