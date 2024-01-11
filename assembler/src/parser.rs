use std::{error::Error, fmt::Display};

use isa::Instruction;

use crate::token::{Keyword, Literal, Punctuation, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserError {
    NumberBadFormat,
    Empty,
    InvalidCharacter,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NumberBadFormat => write!(f, "Formato do número inválido!"),
            ParserError::Empty => write!(f, "Entrada de dados vazia."),
            ParserError::InvalidCharacter => write!(f, "Caracter inválido!"),
        }
    }
}

impl Error for ParserError {}

pub struct Parser<'a> {
    input: &'a str,
    stream: Vec<(Token, String)>,
    curr_pos: usize,
    curr_line: usize,
    curr_col: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            stream: Vec::new(),
            curr_pos: 0,
            curr_line: 0,
            curr_col: 0,
        }
    }

    fn inc_line(&mut self) {
        self.curr_line += 1;
        self.curr_col = 0;
    }

    fn consume_left_whitespaces(&mut self) {
        if self.input.starts_with(char::is_whitespace) {
            for (i, c) in self.input.char_indices() {
                match c {
                    c if c == ' ' || c == '\t' || c == '\r' => continue,
                    c if c == '\n' => self.inc_line(),
                    _ => {
                        self.input = &self.input[i..];
                        self.curr_col = i;
                        break;
                    }
                }
            }
        }
    }

    fn next_string(&mut self) -> Option<String> {
        self.consume_left_whitespaces();

        // Verifica palavras do tipo "foo"
        let mut s: String = if self.input.len() > 0 && self.input.chars().next().unwrap() == '"' {
            let mut index = 0;
            let mut flag = false;
            let p = self.input.char_indices().peekable();
            for (i, c) in p {
                index = i;
                if i != 0 {
                    match c {
                        c if c == '"' => match flag {
                            true => flag = false,
                            false => break,
                        },
                        c if c == '\\' => flag = true,
                        _ => flag = false,
                    }
                }
            }

            self.input[..index + 1].to_string()
        } else {
            // Pode ter identificadores do tipo foo_bar
            self.input
                .chars()
                .take_while(|c| !c.is_whitespace() && (*c == '_' || !c.is_ascii_punctuation()))
                .collect()
        };

        return if s.len() > 0 {
            self.input = &self.input[s.len()..];
            Some(s)
        } else if self.input.len() > 0 {
            s = self.input[..1].to_string();
            self.input = &self.input[1..];
            Some(s)
        } else {
            None
        };
    }

    fn next_number(s: String) -> Result<(Token, String), ParserError> {
        if usize::from_str_radix(s.as_str(), 10).is_ok() {
            Ok((Token::Literal(Literal::DecNumber), s))
        } else if s.chars().next().unwrap() == '0' && s.len() > 2 {
            match s.chars().nth(1).unwrap() {
                c if c == 'b' || c == 'B' => match usize::from_str_radix(&s.as_str()[2..], 2) {
                    Ok(_) => Ok((
                        Token::Literal(Literal::BinNumber),
                        s.as_str()[2..].to_string(),
                    )),
                    Err(_) => Err(ParserError::NumberBadFormat),
                },
                c if c == 'x' || c == 'X' => match usize::from_str_radix(&s.as_str()[2..], 16) {
                    Ok(_) => Ok((
                        Token::Literal(Literal::HexNumber),
                        s.as_str()[2..].to_string(),
                    )),
                    Err(_) => Err(ParserError::NumberBadFormat),
                },
                _ => Err(ParserError::NumberBadFormat),
            }
        } else {
            Err(ParserError::NumberBadFormat)
        }
    }

    fn next_word(s: String) -> Result<(Token, String), ParserError> {
        if let Some(kw) = Keyword::from_str(s.as_str()) {
            Ok((Token::Keyword(kw), s))
        } else if let Some(i) = Instruction::from_str(s.as_str()) {
            Ok((Token::Instruction(i), s))
        } else {
            Ok((Token::Identifier, s))
        }
    }

    fn next_punctuation(s: String) -> Result<(Token, String), ParserError> {
        if let Some(p) = Punctuation::from_str(s.as_str()) {
            Ok((Token::Punctuation(p), s))
        } else {
            Err(ParserError::InvalidCharacter)
        }
    }

    pub fn next_token(&mut self) -> Result<(Token, String), ParserError> {
        match self.next_string() {
            Some(w) => match w.chars().next().unwrap() {
                c if c.is_digit(10) => Parser::next_number(w),
                c if c.is_alphabetic() => Parser::next_word(w),
                c if c == '"' => Ok((
                    Token::Literal(Literal::String),
                    w.as_str()[1..w.len() - 1].to_string(),
                )),
                _ => Parser::next_punctuation(w),
            },
            None => Err(ParserError::Empty),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_consume_left_whitespace() {
        let input = "   \n\r  Test   ";
        let mut p = Parser::new(&input);
        p.consume_left_whitespaces();

        assert_eq!("Test   ", p.input);
    }

    #[test]
    fn test_next_string_1() {
        let input = "\n\tTest";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_string().unwrap());
    }

    #[test]
    fn test_next_string_2() {
        let input = "\n\tTest 2Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_string().unwrap());
    }

    #[test]
    fn test_next_string_3() {
        let input = "\n\tTest:3Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_string().unwrap());
    }

    #[test]
    fn test_next_string_4() {
        let input = "\n\tTest_4Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test_4Test".to_string(), p.next_string().unwrap());
    }

    #[test]
    fn test_next_string_5() {
        let input = "\n\tTest 5Test";
        let mut p = Parser::new(&input);

        p.next_string();
        assert_eq!("5Test".to_string(), p.next_string().unwrap());
    }

    #[test]
    fn test_next_string_6() {
        let input = r#" "Test 6""#;
        let mut p = Parser::new(&input);

        assert_eq!("\"Test 6\"".to_string(), p.next_string().unwrap());
    }

    #[test]
    fn test_next_token_literal_string_1() {
        let input = r#"  "Test literal" 123"#;
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Literal(Literal::String), "Test literal".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_literal_dec() {
        let input = " 123456 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Literal(Literal::DecNumber), "123456".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_literal_bin() {
        let input = " 0b01011 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Literal(Literal::BinNumber), "01011".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_literal_hex() {
        let input = " 0xffc0 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Literal(Literal::HexNumber), "ffc0".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_keyword() {
        let input = " var   r0 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Keyword(Keyword::Var), "var".to_string()),
            p.next_token().unwrap()
        );

        assert_eq!(
            (Token::Keyword(Keyword::R0), "r0".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_instruction() {
        let input = " mov   r0 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Instruction(Instruction::MOV), "mov".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_identifier() {
        let input = " label: mov   r0 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Identifier, "label".to_string()),
            p.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_punctuation() {
        let input = "  ,r0   r0 Test";
        let mut p = Parser::new(&input);

        assert_eq!(
            (Token::Punctuation(Punctuation::Comma), ",".to_string()),
            p.next_token().unwrap()
        );
    }
}
