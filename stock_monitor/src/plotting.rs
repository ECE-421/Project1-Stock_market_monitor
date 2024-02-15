/*
Written by Devin Headrick
For ECE421 Project 1 - Stock market monitor
Winter 2024 Semester
Univserity of Alberta

Referenced: https://tms-dev-blog.com/plot-candles-sma-using-rust-and-plotters/

NOTES:
- Quote structs returned by the yahoo finance api use timestampes in UNIX time (miliseconds since epoch)
- Plotters lib depends on the following libs to be installed: pkg-config libfreetype6-dev libfontconfig1-dev
    - Install using: sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev
*/
//TODO - Fill in candlesticks
//TODO - Get plot working on a web server or interactive GUI (not just image gen backend)
//TODO - Plot bollinger bands
//TODO - Use logger in place of print

use chrono::{NaiveDate, NaiveDateTime};
use plotters::{prelude::*, style::full_palette::PURPLE};
use tokio_test;
use yahoo_finance_api as yahoo;

use crate::utils::convert_UNIX_timestamp_to_naivedate;

use super::utils::{
    calc_simple_moving_average, get_min_max_closing_prices, get_start_end_dates, is_volatile_day,
    get_max_min_closing_quotes,
};

const CANDLE_STICK_OUT_FILE_NAME: &str = "./candlestick_plot.png";
const SMA_PLOT_OUT_FILE_NAME: &str = "./sma_plot.png";
const OVERLAY_PLOT_OUT_FILE_NAME: &str = "./candlestick_sma_overlay_plot.png";

const CAPTION_FONT: &str = "sans-serif";

const PLOT_WIDTH: u32 = 1024;
const PLOT_HEIGHT: u32 = 768;
const CANDLE_STICK_WIDTH: u32 = 9;

const VOLATILE_PERCENTAGE_THRESHOLD: f64 = 0.02; //2% As per project handout
const SMA_WINDOW_SIZE: usize = 10;

// Generates a plot for the provided ticker, showing the SMA and candlesticks for each intra day price. 
// Volatile days are highlighted blue. Days with net increase in closing price highlighted green. Days with net decrease in closing price highlighted red
pub fn make_candlestick_and_sma_plot(
    ticker: &str,
    quotes: &Vec<yahoo_finance_api::Quote>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Generating SMA and Candlestick overlay plot for {}...",
        ticker
    );

    let date_range = get_start_end_dates(quotes);
    let price_range = get_min_max_closing_prices(quotes);

    let root = BitMapBackend::new(OVERLAY_PLOT_OUT_FILE_NAME, (PLOT_WIDTH, PLOT_HEIGHT))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let caption = format!("{} - SMA & Candlestick Plot", ticker);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(caption, (CAPTION_FONT, 50.0).into_font())
        .margin(20)
        .build_cartesian_2d(date_range.0..date_range.1, price_range.0..price_range.1)?;

    chart.configure_mesh()
    .x_desc("Date  [YYYY-MM-DD]")
    .y_desc("Price [USD]")
    .x_label_style(("sans-serif", 15).into_font())
    .y_label_style(("sans-serif", 15).into_font())
    .draw()?;

    //Populate the chart with the data from the quotes vector
    chart
        .draw_series(
            quotes.iter().map(|quote| {
                let x = NaiveDateTime::from_timestamp_opt(quote.timestamp as i64, 0)
                    .unwrap()
                    .date();
                // Prices are rounded to 2 decimal places
                let open = (quote.open * 100.0).round() / 100.0;
                let close = (quote.close * 100.0).round() / 100.0;
                let high = (quote.high * 100.0).round() / 100.0;
                let low = (quote.low * 100.0).round() / 100.0;

                if is_volatile_day(quote, VOLATILE_PERCENTAGE_THRESHOLD) {
                    CandleStick::new(x, open, high, low, close, BLUE, BLUE, CANDLE_STICK_WIDTH+2)
                } else {
                    CandleStick::new(x, open, high, low, close, GREEN, RED, CANDLE_STICK_WIDTH)
                }
            }),
        )?
        .legend(|(x, y)| PathElement::new(vec![(x, y ), (x , y )], &BLUE));

    //DRAW SMA
    let closing_prices: Vec<f64> = quotes.iter().map(|quote| quote.close).collect();
    let sma_data = calc_simple_moving_average(&closing_prices, SMA_WINDOW_SIZE).unwrap();

    let mut line_data: Vec<(NaiveDate, f64)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((
            NaiveDateTime::from_timestamp_opt(quotes[i].timestamp as i64, 0)
                .unwrap()
                .date(),
            sma_data[i] as f64,
        ));
    }

    chart
        .draw_series(LineSeries::new(line_data, PURPLE.stroke_width(3)))
        .unwrap()
        .label("Simple Moving Average (SMA)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x+10, y)], &PURPLE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .label_font(("sans-serif", 20.0).into_font())
        .background_style(&WHITE.mix(0.0))
        .draw()
        .unwrap();

    root.present()
        .expect("Unable to save result to file. Enusre ");
    println!("Plot saved to {}", OVERLAY_PLOT_OUT_FILE_NAME);

    Ok(())
}

pub fn make_candlestick_plot(
    ticker: &str,
    quotes: &Vec<yahoo_finance_api::Quote>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating Candlestick plot for {}", ticker);

    let root = BitMapBackend::new(CANDLE_STICK_OUT_FILE_NAME, (PLOT_WIDTH, PLOT_HEIGHT))
        .into_drawing_area();
    root.fill(&WHITE)?;

    // Functionally get the min and max UTC time value (nano seconds since epoch) from the quotes vec
    let min_time = quotes.iter().map(|quote| quote.timestamp).min().unwrap() as i64;
    let max_time = quotes.iter().map(|quote| quote.timestamp).max().unwrap() as i64;

    //Need to create date objects from UNIX epoch seconds
    let min_date = NaiveDateTime::from_timestamp_opt(min_time, 0)
        .unwrap()
        .date();
    let max_date = NaiveDateTime::from_timestamp_opt(max_time, 0)
        .unwrap()
        .date();

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

    let caption = format!("{} - Candlestick Plot", ticker);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(caption, (CAPTION_FONT, 50.0).into_font())
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    //Populate the chart with the data from the quotes vector
    chart
        .draw_series(
            //Iterate on quote vector and create candlestick
            quotes.iter().map(|quote| {
                let x = NaiveDateTime::from_timestamp_opt(quote.timestamp as i64, 0)
                    .unwrap()
                    .date();

                // Prices are rounded to 2 decimal places
                let open = (quote.open * 100.0).round() / 100.0;
                let close = (quote.close * 100.0).round() / 100.0;
                let high = (quote.high * 100.0).round() / 100.0;
                let low = (quote.low * 100.0).round() / 100.0;
                CandleStick::new(x, open, high, low, close, GREEN, RED, 10)
            }),
        )
        .unwrap();

    root.present()
        .expect("Unable to save result to file. Enusre ");
    println!("Plot saved to {}", CANDLE_STICK_OUT_FILE_NAME);

    Ok(())
}

pub fn make_sma_plot(
    ticker: &str,
    quotes: &Vec<yahoo_finance_api::Quote>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating Candlestick plot for {}", ticker);

    let root =
        BitMapBackend::new(SMA_PLOT_OUT_FILE_NAME, (PLOT_WIDTH, PLOT_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define times for x-axis
    let min_time = quotes.iter().map(|quote| quote.timestamp).min().unwrap() as i64;
    let max_time = quotes.iter().map(|quote| quote.timestamp).max().unwrap() as i64;

    let closing_prices: Vec<f64> = quotes.iter().map(|quote| quote.close).collect();
    let sma_data = calc_simple_moving_average(&closing_prices, 20).unwrap();

    //Define min and max prices for y-axis
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

    let caption = format!("{} - Simple Moving Average (SMA) Plot", ticker);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(caption, (CAPTION_FONT, 50.0).into_font())
        .build_cartesian_2d(min_time..max_time, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    let mut line_data: Vec<(i64, f64)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((quotes[i].timestamp as i64, sma_data[i] as f64));
    }

    chart
        .draw_series(LineSeries::new(line_data, BLUE.stroke_width(2)))
        .unwrap()
        .label("Simple Moving Average (SMA) [entire timespan]")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperMiddle)
        .label_font(("sans-serif", 30.0).into_font())
        .background_style(WHITE.filled())
        .draw()
        .unwrap();

    root.present()
        .expect("Unable to save result to file. Enusre ");
    println!("Plot saved to {}", SMA_PLOT_OUT_FILE_NAME);

    Ok(())
}

pub fn print_min_max_closing_prices_and_date(quotes: &Vec<yahoo_finance_api::Quote>) {
    let min_max_quotes = get_max_min_closing_quotes(quotes);
    println!("Min closing price: {}, occured on {}", min_max_quotes.0.close, convert_UNIX_timestamp_to_naivedate(min_max_quotes.0.timestamp));
    println!("Max closing price: {}, occured on {}", min_max_quotes.1.close, convert_UNIX_timestamp_to_naivedate(min_max_quotes.1.timestamp));
}


fn get_data() -> Vec<yahoo_finance_api::Quote> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "6mo")).unwrap();
    let quotes = response.quotes().unwrap();
    quotes
}

pub fn test_candlestick_and_sma_plot(ticker: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stock_data = get_data();
    let test_candlestick_and_sma_plot = make_candlestick_and_sma_plot(ticker, &stock_data);
    test_candlestick_and_sma_plot
}

pub fn test_print_min_max_closing_price_and_date() {
    let stock_data = get_data();
    print_min_max_closing_prices_and_date(&stock_data);
}
