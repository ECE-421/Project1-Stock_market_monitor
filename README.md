# Project1-Stock_market_monitor

ECE421 project 1 - Gather stock data based on stock ticker symbol. Based entirely in Rust.

# Crates Used

-   clap: this is a Rust crate used for parsing command line arguments, and it also automatically generates comprehensive help and usage messages, enhancing user experience and guidance. In this project, we used it to read the stock ticker.

- yahoo_finance_api: This Rust crate is used as an adapter to interact with the yahoo_finance_api through rust application. The api allows us to get information realting to relevant stock info. In this project it is used to return Quotes from a specified ticker symbol to get info on the opening, high, low, and closing prices. We specifically chose this crate over other api's as this one was the one we seen that was most up to date.

- plotters: This rust crate is used to generate our finnacial plots. This crate was chosen as it seems to be the go to for Rust with good documentation and actively being updated.

# The financial analysis algorithm you used.

## Volatile days formula

To determine if a trading day is volatile, we can use the following mathematical formula in Rust:

$\[ \text{{Percentage Change}} = \frac{{\left| \text{{high}} - \text{{low}} \right|}}{{\text{{close}}}} \]$

- `high`: The highest price during the trading day.
- `low`: The lowest price during the trading day.
- `close`: The closing price of the trading day.

if this is > than our threshold (0.02) the day is considered volatile

Rust Implementation:
The function `is_volatile_day` returns `true` if the calculated percentage change is greater than the specified threshold, indicating a volatile day.

```rust
pub fn is_volatile_day(quote: &Quote, percentage_threshold: f64) -> bool {
    let high = quote.high;
    let low = quote.low;
    let close = quote.close;
    let price_difference = (high - low).abs();
    let percentage_change = price_difference / close;

    percentage_change > percentage_threshold
}
```

## Simple Moving Average 


$\[ SMA = \frac{{X_1 + X_2 + X_3 + \ldots + X_n}}{{n}} \]$

Where:
- $\( X_1, X_2, X_3, \ldots, X_n \)$ are the individual data points in the time series.
- $\( n \)$ is the number of data points considered in the calculation.

```rust
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
```


# Chart Setup

We wanted to display as much relevant stock info as possible so we have 3 main plots to show information a user may want. All 3 plots use the same autoscaling and axis features which take the maximum price and date range with a slight offset to correctly fit the entire plot.

1. Closing Price With Volatile Days
    This plot is a direct copy of what is asked in the assignment pdf. It uses a line plot to show the closing prices for the last 6 months while highlighting the volatile days with the interday high and low as error bars.

2. Simple Moving Average
    The simple moving average is a pretty standard plot for any stock plot as it can give indicators on whether or not the stock price will fall/continue on its trend. This plot is a Line series plot.

3. Candle Stick Plot showing the SMA and candlesticks for each intra day price. Volatile days are highlighted blue. Days with net increase in closing price highlighted green. Days with net decrease in closing price highlighted red


# Project Setup

1. clone the repo
```git clone git@github.com:ECE-421/Project1-Stock_market_monitor.git```

2. Navigate to the stock_monitor folder
3. To install the nessecarry rust crates and build the project run  
   ```cargo build```

# Usage Instructions

1. Once built run the program using
   ```cargo run <ticker symbol>```
2. 3 photo plots will be generated based on the entered ticker symbol
