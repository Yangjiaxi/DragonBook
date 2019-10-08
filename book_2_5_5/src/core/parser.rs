use super::{Chars, ParserError};

#[derive(Debug)]
pub struct Parser<'a> {
    lookahead: Option<char>,
    input_iter: Chars<'a>,
}

impl Parser<'_> {
    pub fn init(input: &'static str) -> Self {
        let mut it = input.chars();
        let lh = it.next();
        Parser {
            input_iter: it,
            lookahead: lh,
        }
    }

    pub fn expr(&mut self) -> Result<(), ParserError> {
        self.term()?;
        loop {
            match self.lookahead {
                Some('+') => {
                    self.can_match('+')?;
                    self.term()?;
                    println!("Opt \t: +");
                }
                Some('-') => {
                    self.can_match('-')?;
                    self.term()?;
                    println!("Opt \t: -");
                }
                Some(c) => {
                    return Err(format!("Syntax Error, Unexpected char `{}` ", c))?;
                }
                _ => return Ok(()),
            }
        }
    }

    pub fn term(&mut self) -> Result<(), ParserError> {
        match self.lookahead {
            Some(x) if x.is_ascii_digit() => {
                println!("Term\t: {}", x);
                return self.can_match(x);
            }
            Some(x) => {
                return Err(format!("Syntax Error, Unexpected term `{}` ", x))?;
            }
            _ => {
                return Err("Unknown Error")?;
            }
        };
    }

    pub fn can_match(&mut self, target: char) -> Result<(), ParserError> {
        match self.lookahead {
            Some(x) if x == target => {
                self.lookahead = self.input_iter.next();
                return Ok(());
            }
            _ => {
                return Err("Syntax Error")?;
            }
        };
    }
}
