mod command_line_parser; 
fn main() {
    let stock_ticker = command_line_parser::read_stock_ticker();
    if command_line_parser::check_stock_ticker(stock_ticker) {
        // api
        // plot
    }

}
