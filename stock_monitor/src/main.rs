mod yahoo_api;

fn main() {
   let symbol = "AAPaL";

   let quotes = yahoo_api::get_quote_history(symbol, 6);
   println!("Quotes for {}: {:?}", symbol, quotes);
}
