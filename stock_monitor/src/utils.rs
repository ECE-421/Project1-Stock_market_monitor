/*
Written by Devin Headrick
For ECE421 Project 1 - Stock market monitor
Winter 2024 Semester
Univserity of Alberta

Referenced: https://tms-dev-blog.com/plot-candles-sma-using-rust-and-plotters/

NOTES:
- Util functions for the stock_monitor project

*/

use yahoo_finance_api as yahoo;
use yahoo::Quote;

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
