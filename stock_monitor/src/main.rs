use crate::plotting::{make_candlestick_and_sma_plot, print_min_max_closing_prices_and_date};

mod plotting;
mod utils;
mod command_line_parser; 
mod yahoo_api;

fn main() {
   let stock_ticker = command_line_parser::read_stock_ticker();
   if command_line_parser::check_stock_ticker(stock_ticker.clone()) {
      let quotes = yahoo_api::get_quote_history(&stock_ticker, 6);
      
      if quotes.is_empty() {
         println!("The entered stock ticker of {}, may not exist", stock_ticker);
      } else {
         if let Err(err) = make_candlestick_and_sma_plot(stock_ticker.as_str(), &quotes) {
            println!("Error occurred while plotting: {}", err);
         }
         print_min_max_closing_prices_and_date(&quotes);
      }
   }
}
