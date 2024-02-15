
mod plotting;

mod utils;


fn main() {
    let ticker: &str = "AAPL";
    // plotting::test_plot(ticker);
    // plotting::test_sma_plot(ticker);
    plotting::test_candlestick_and_sma_plot(ticker);
    plotting::test_print_min_max_closing_price_and_date();

}