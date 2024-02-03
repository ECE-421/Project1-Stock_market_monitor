/*
Written by Devin Headrick
For ECE421 Project 1 - Stock market monitor
Winter 2024 Semester
Univserity of Alberta
*/

use chrono::offset::{Local, TimeZone};
// use chrono::{Date, Duration};

use plotters::prelude::*;

use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use tokio_test;
use yahoo_finance_api as yahoo;

// What does the yahoo finance api return?

const OUT_FILE_NAME: &str = "./stock.png";

//Create plot from a vector of 'quote' structs

pub fn make_plot(quotes: Vec<yahoo_finance_api::Quote>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from make_plot");

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let start_time = quotes[0].timestamp as i64;
    let end_time = quotes[quotes.len() - 1].timestamp as i64;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("CHART CAPTION HERE", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(start_time..end_time, 110f64..135f64)?;

    chart.configure_mesh().draw()?;

    //Populate the chart with the data from the quotes vector
    chart.draw_series(
        //Iterate on quote vector and create candlestick
        quotes.iter().map(|quote| {
            let time: OffsetDateTime = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
            
            // format open, close, high, low, to 2 decimal places
            let x = time.timestamp();
            let open = (quote.open * 100.0).round() / 100.0;
            let close = (quote.close * 100.0).round() / 100.0;
            let high = (quote.high * 100.0).round() / 100.0;
            let low = (quote.low * 100.0).round() / 100.0;

            //print all the values
            println!(
                "Time: {:?}, Open: {}, Close: {}, High: {}, Low: {}",
                time, open, close, high, low
            );

            //Create a candlestick
            CandleStick::new(x, open, high, low, close, GREEN.filled(), RED, 15)
        }),
    )?;

    /*
    Candlestick constructor takes:
    x: X,
        open: Y,
        high: Y,
        low: Y,
        close: Y,
        gain_style: GS,
        loss_style: LS,
        width: u32,
     */


    //Need to figure out what the timestamp format is used by the Quote struct

    


    root.present()
        .expect("Unable to save result to file. Enusre ");
    println!("Plot saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn get_data() -> Vec<yahoo_finance_api::Quote> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    println!("Apple's quotes of the last month: {:?}", quotes);

    quotes
}

pub fn test_plot() -> Result<(), Box<dyn std::error::Error>> {
    let stock_data = get_data();
    let plot_test = make_plot(stock_data);

    Ok(())
}
