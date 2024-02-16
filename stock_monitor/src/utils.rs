/*
Written by Devin Headrick,
For ECE421 Project 1 - Stock market monitor
Winter 2024 Semester
Univserity of Alberta

Referenced: https://tms-dev-blog.com/plot-candles-sma-using-rust-and-plotters/

NOTES:
- Util functions for the stock_monitor project

*/

use chrono::{NaiveDate, NaiveDateTime};
use yahoo::Quote;
use yahoo_finance_api as yahoo;

pub fn calc_simple_moving_average(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }
    let mut result: Vec<f64> = Vec::new();
    let mut window_start = 0;
    while window_start + window_size <= data_set.len() {
        let window_end = window_start + window_size;
        let data_slice = &data_set[window_start..window_end];
        let sum: f64 = data_slice.iter().sum();
        let average = sum / window_size as f64;
        result.push(average);
        window_start += 1;
    }
    Some(result)
}

pub fn is_volatile_day(quote: &Quote, percentage_threshold: f64) -> bool {
    let high = quote.high;
    let low = quote.low;
    let close = quote.close;
    let price_difference = (high - low).abs();
    let percentage_change = price_difference / close;

    percentage_change > percentage_threshold
}

pub fn get_min_max_closing_prices(quotes: &Vec<Quote>) -> (f64, f64) {
    let min_price = quotes
        .iter()
        .map(|quote| quote.low)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_price = quotes
        .iter()
        .map(|quote| quote.high)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    (min_price, max_price)
}

pub fn convert_UNIX_timestamp_to_naivedate(timestamp: u64) -> NaiveDate{
    NaiveDateTime::from_timestamp_opt(timestamp as i64,0).unwrap().date()
}

pub fn get_start_end_dates(quotes: &Vec<Quote>) -> (NaiveDate, NaiveDate) {
    // Functionally get the min and max UTC time value (nano seconds since epoch) from the quotes vec
    let min_time = quotes.iter().map(|quote| quote.timestamp).min().unwrap() as i64;
    let max_time = quotes.iter().map(|quote| quote.timestamp).max().unwrap() as i64;

    let min_date = NaiveDateTime::from_timestamp_opt(min_time, 0)
        .unwrap()
        .date();
    let max_date = NaiveDateTime::from_timestamp_opt(max_time, 0)
        .unwrap()
        .date();

    (min_date, max_date)
}

pub fn get_max_min_closing_quotes(quotes: &Vec<Quote>) -> (Quote, Quote) {
    let min_quote = quotes
        .iter()
        .min_by(|a, b| a.close.partial_cmp(&b.close).unwrap())
        .unwrap();
    let max_quote = quotes
        .iter()
        .max_by(|a, b| a.close.partial_cmp(&b.close).unwrap())
        .unwrap();

    (min_quote.clone(), max_quote.clone())
}

pub fn find_volatile_days(quotes: &[Quote], percentage_threshold: f64) -> Vec<&Quote> {
    quotes
        .iter()
        .filter(|quote| {
            let high = quote.high;
            let low = quote.low;
            let close = quote.close;
            let price_difference = (high - low).abs();
            let percentage_change = price_difference / close;

            percentage_change > percentage_threshold
        })
        .collect()
}
