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
    ticker.len() >= 3 && ticker.len() <= 5
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



