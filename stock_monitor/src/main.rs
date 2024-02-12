mod command_line_parser; 
mod yahoo_api;

fn main() {
   let stock_ticker = command_line_parser::read_stock_ticker();
   if command_line_parser::check_stock_ticker(stock_ticker.clone()) {
      let quotes = yahoo_api::get_quote_history(&stock_ticker, 6);
      
      if quotes.is_empty() {
         println!("The entered stock ticker of {}, may not exist", stock_ticker);
      } else {
         println!("Quotes for {}: {:?}", &stock_ticker, quotes);
      }
      
       // api
       // plot
   }
}
