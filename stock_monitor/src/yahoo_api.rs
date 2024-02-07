use yahoo_finance_api as yahoo;
use time::{macros::datetime, OffsetDateTime};
use tokio_test;


pub fn get_quote_history(ticker_symbol: &str, month_offset: i64 ) -> Vec<yahoo::Quote> {
   let end = OffsetDateTime::now_utc();
   let start = end - time::Duration::days(30 * month_offset); // if month_offset = 6 it would be 30 * 6 days

   let provider = yahoo::YahooConnector::new();
   match tokio_test::block_on(provider.get_quote_history(ticker_symbol, start, end)) {
      Ok(resp) => {
         let quotes = resp.quotes().unwrap();
         return quotes
      }
      Err(err) => {
         eprintln!("Error: {:?}", err);
         return Vec::new()
      }
   }
}