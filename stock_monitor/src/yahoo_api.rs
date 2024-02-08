use yahoo_finance_api as yahoo;
use time::OffsetDateTime;
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

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_get_quote_history_valid() {
        // Provide a valid ticker symbol and month offset for testing
        let ticker_symbol = "AAPL";
        let month_offset = 6;

        let quotes = get_quote_history(ticker_symbol, month_offset);

        assert!(!quotes.is_empty());

    }

    #[tokio::test]
    async fn test_get_quote_history_invalid_ticker() {
        let invalid_ticker = "INVALID";

        let quotes = get_quote_history(invalid_ticker, 3);

        assert!(quotes.is_empty());
    }
}
