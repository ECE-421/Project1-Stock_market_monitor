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

//TODO - Fix dates on x-axis to be human readable (not UNIX time stamps)
//TODO - Update chart title
//TODO - Identify and Highlight volatile days different color
//TODO - Plot bollinger bands
//TODO - Get plot working on a web server or interactive GUI (not just image gen backend)

use plotters::prelude::*;
use tokio_test;
use yahoo_finance_api as yahoo;

// import the calc_simple_moving_average function from the utils module
use super::utils::calc_simple_moving_average;

const CANDLE_STICK_OUT_FILE_NAME: &str = "./candlestick_plot.png";
const SMA_PLOT_OUT_FILE_NAME: &str = "./sma_plot.png";

pub fn make_candlestick_plot(
    quotes: &Vec<yahoo_finance_api::Quote>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating SMA plot...");

    let root = BitMapBackend::new(CANDLE_STICK_OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_time = quotes.iter().map(|quote| quote.timestamp).min().unwrap() as i64;
    let max_time = quotes.iter().map(|quote| quote.timestamp).max().unwrap() as i64;

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

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("CHART CAPTION HERE", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(min_time..max_time, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    //Populate the chart with the data from the quotes vector
    chart
        .draw_series(
            //Iterate on quote vector and create candlestick
            quotes.iter().map(|quote| {
                let x = quote.timestamp as i64;
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
    quotes: &Vec<yahoo_finance_api::Quote>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating SMA plot...");

    let root = BitMapBackend::new(SMA_PLOT_OUT_FILE_NAME, (1024, 768)).into_drawing_area();
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

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("CHART CAPTION HERE", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(min_time..max_time, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    //Populate the chart with the data from the quotes vector

    let mut line_data: Vec<(i64, f64)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((quotes[i].timestamp as i64, sma_data[i] as f64));
    }

    chart
        .draw_series(LineSeries::new(line_data, BLUE.stroke_width(2)))
        .unwrap()
        .label("SMA Plot")
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

fn get_data() -> Vec<yahoo_finance_api::Quote> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "6mo")).unwrap();
    let quotes = response.quotes().unwrap();
    quotes
}

pub fn test_plot() -> Result<(), Box<dyn std::error::Error>> {
    let stock_data = get_data();
    let plot_test = make_candlestick_plot(&stock_data);

    plot_test
}

pub fn test_sma_plot() -> Result<(), Box<dyn std::error::Error>> {
    let stock_data = get_data();
    let test_sma_plot = make_sma_plot(&stock_data);
    test_sma_plot
}
