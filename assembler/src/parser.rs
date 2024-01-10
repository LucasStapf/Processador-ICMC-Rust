use std::{error::Error, fmt::Display};

use crate::token::{Literal, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserError {
    NumberBadFormat,
    Empty,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NumberBadFormat => write!(f, "Formato do número inválido!"),
            ParserError::Empty => write!(f, "Entrada de dados vazia."),
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

    fn next_word(&mut self) -> Option<String> {
        self.consume_left_whitespaces();

        let s: String = if self.input.len() > 0 && self.input.chars().next().unwrap() == '"' {
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

            self.input[1..index].to_string()
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
        } else {
            None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_consume_left_whitespace() {
        let input = "   \n\r  Test   ";
        let mut p = Parser::new(&input);
        p.consume_left_whitespaces();

        assert_eq!("Test   ", p.input);
    }

    #[test]
    fn test_next_word_1() {
        let input = "\n\tTest";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_word().unwrap());
    }

    #[test]
    fn test_next_word_2() {
        let input = "\n\tTest 2Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_word().unwrap());
    }

    #[test]
    fn test_next_word_3() {
        let input = "\n\tTest:3Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_word().unwrap());
    }

    #[test]
    fn test_next_word_4() {
        let input = "\n\tTest_4Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test_4Test".to_string(), p.next_word().unwrap());
    }

    #[test]
    fn test_next_word_5() {
        let input = "\n\tTest 5Test";
        let mut p = Parser::new(&input);

        p.next_word();
        assert_eq!("5Test".to_string(), p.next_word().unwrap());
    }

    #[test]
    fn test_next_word_6() {
        let input = r#" "Test 6""#;
        let mut p = Parser::new(&input);

        assert_eq!("Test 6".to_string(), p.next_word().unwrap());
    }
}
