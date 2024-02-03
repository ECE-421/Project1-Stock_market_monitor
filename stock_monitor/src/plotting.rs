/*
Written by Devin Headrick
For ECE421 Project 1 - Stock market monitor
Winter 2024 Semester
Univserity of Alberta

Referenced: https://tms-dev-blog.com/plot-candles-sma-using-rust-and-plotters/

NOTES:
- Quote structs returned by the yahoo finance api use timestampes in UNIX time (miliseconds since epoch)

*/
use plotters::prelude::*;
use tokio_test;
use yahoo_finance_api as yahoo;

const OUT_FILE_NAME: &str = "./stock.png";

pub fn make_plot(quotes: Vec<yahoo_finance_api::Quote>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating plot...");

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
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

                //print all the values
                // println!(
                //     "Time: {:?}, Open: {}, Close: {}, High: {}, Low: {}",
                //     x, open, close, high, low
                // );
                CandleStick::new(x, open, high, low, close, GREEN.filled(), RED, 15)
            }),
        )
        .unwrap();

    root.present()
        .expect("Unable to save result to file. Enusre ");
    println!("Plot saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn get_data() -> Vec<yahoo_finance_api::Quote> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    // println!("Apple's quotes of the last month: {:?}", quotes);

    quotes
}

pub fn test_plot() -> Result<(), Box<dyn std::error::Error>> {
    let stock_data = get_data();
    let plot_test = make_plot(stock_data);

    plot_test
}
