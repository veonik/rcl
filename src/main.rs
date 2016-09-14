extern crate rcl;

use rcl::parse::lexer::Lexer;
use rcl::parse::token::Token;

fn main() {
    let mut l = Lexer::new("$some input".to_string());
    let t = l.next();

    match t {
        Some(Token::Var { name: e }) => println!("Token: {}", e),
        None => println!("No token!"),
        _ => println!("Unmatched!")
    }
}