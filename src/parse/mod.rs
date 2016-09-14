pub mod lexer;
pub mod token;

use self::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

pub enum ExpressionType {
    Block,
    String,
}

pub struct Expression {

}

pub struct ExpressionSet {

}

impl Parser {
    fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input)
        }
    }

    fn parse(&self) -> ExpressionSet {
        unimplemented!()
    }
}