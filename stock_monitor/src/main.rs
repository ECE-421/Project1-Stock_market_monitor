mod command_line_parser; 
mod yahoo_api;

fn main() {
   let symbol = "AAPaL";

   let quotes = yahoo_api::get_quote_history(symbol, 6);
   println!("Quotes for {}: {:?}", symbol, quotes);

   let stock_ticker = command_line_parser::read_stock_ticker();
   if command_line_parser::check_stock_ticker(stock_ticker) {
       // api
       // plot
   }
}
