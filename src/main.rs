extern crate rcl;

use rcl::parse::lexer::Lexer;
use rcl::parse::token::Token;

fn main() {
    let mut l = Lexer::new("proc square {x} {
    * $x $x
}

set a 1
puts \"$a^2 = [square $a]\"".to_string());

    loop {
        let t = l.next();
        match t {
            Some(Token::Var { name: e }) => println!("Var: {}", e),
            Some(Token::Word { contents: e }) => println!("Word: {}", e),
            Some(Token::Whitespace { contents: e }) => println!("Space: {}", e),
            Some(_) => println!("Another token: {:?}", t.unwrap()),
            None => {
                println!("End of input");
                break
            }
        }
    }
}