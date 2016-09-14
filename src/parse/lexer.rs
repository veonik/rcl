use regex::Regex;
use parse::token::Token;

pub struct Lexer {
    input: String,
    pos: usize,
    match_space: Regex,
    match_word: Regex
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            pos: 0usize,
            match_space: Regex::new("\\s").unwrap(),
            match_word: Regex::new("[0-9a-zA-Z\\$]").unwrap()
        }
    }

    pub fn peek(&self, size: usize) -> &str {
        &self.input[self.pos..(self.pos+size)]
    }

    fn end_of_word(&self) -> usize {
        let mut c = self.pos;
        while c < self.input.len() {
            if !self.match_word.is_match(&self.input[c..c+1]) {
                break;
            }
            c += 1;
        }
        c
    }

    fn end_of_whitespace(&self) -> usize {
        0usize
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek(1) {
            "$" => {
                let e = self.end_of_word();
                let name = &self.input[self.pos..(self.pos+e)];
                self.pos = e;
                Some(Token::Var{ name: name.to_string() })
            },
            _ => {
                None
            }
        }
    }
}