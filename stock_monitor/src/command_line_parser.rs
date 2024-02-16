use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub stock_ticker: String,
}

fn parse_args() -> Args {
    Args::parse()
}


pub fn read_stock_ticker() -> String {
    let args: Args = parse_args();
    args.stock_ticker.to_uppercase()
}

fn check_stock_ticker_length(ticker: &str) -> bool {
    ticker.len() >= 1 && ticker.len() <= 5
}

fn check_stock_ticker_alphabetic(ticker: &str) -> bool {
    ticker.chars().all(|c: char| c.is_alphabetic())
}

pub fn check_stock_ticker(ticker: String) -> bool {
    if !check_stock_ticker_length(&ticker) {
        println!("Invalid ticker: Length should be between 3 and 5 characters.");
        return false;
    }
    if !check_stock_ticker_alphabetic(&ticker) {
        println!("Invalid ticker: Should contain only alphabetic characters.");
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_stock_ticker_lenth_invalid() {
        assert!(!check_stock_ticker_length("as"));
        assert!(!check_stock_ticker_length("qwerty"));

    }

    #[test]
    fn test_check_stock_ticker_lenth_valid() {
        assert!(check_stock_ticker_length("asd"));
        assert!(check_stock_ticker_length("asder"));
    }

    #[test]
    fn test_check_stock_ticker_alphabetic_valid() {
        assert!(check_stock_ticker_alphabetic("ABC"));
        assert!(check_stock_ticker_alphabetic("XYZ"));
        assert!(check_stock_ticker_alphabetic("WXYZ"));
    }

    #[test]
    fn test_check_stock_ticker_alphabetic_invalid() {
        assert!(!check_stock_ticker_alphabetic("123"));
        assert!(!check_stock_ticker_alphabetic("ABC123"));
        assert!(!check_stock_ticker_alphabetic("ABC!"));
    }


}

