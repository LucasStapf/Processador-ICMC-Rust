use std::str::FromStr;

use crate::token::{self, Token, TokenError, PUNCTUATION};

pub const COMMENTATY_BEGIN: char = ';';

pub struct Lexer<'a> {
    stream: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Self {
            stream,
            line: 1,
            column: 1,
        }
    }

    pub fn stream(&self) -> &str {
        self.stream
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    fn next_line(&mut self) {
        self.stream = match self.stream.split_once(|c| c == '\n') {
            Some((_, s)) => {
                self.increment_line();
                s
            }
            None => "",
        }
    }

    fn trim_left(&mut self) {
        let mut n = 0;

        for (i, c) in self.stream.char_indices() {
            match c {
                '\n' => {
                    n = i;
                    self.increment_line();
                }
                c if c.is_whitespace() => {
                    n = i;
                    self.column += 1;
                }
                _ => {
                    n = i;
                    break;
                }
            }
        }

        self.stream = &self.stream[n..];
    }

    fn skip_commentary(&mut self) {
        while let Some(COMMENTATY_BEGIN) = self.stream.chars().next() {
            self.next_line();
            self.trim_left();
        }
    }

    fn string_item(s: &str) -> String {
        let mut iter = s.chars();
        let mut ret_val = String::with_capacity(s.len());
        let mut count = 0;
        while let Some(c) = iter.next() {
            match c {
                '\\' => match iter.next() {
                    Some(c) => ret_val.push(c),
                    None => break,
                },
                '"' => {
                    count += count;
                    ret_val.push(c);
                    if count == 2 {
                        break;
                    }
                }
                _ => ret_val.push(c),
            }
        }

        ret_val
    }

    fn next_item(&mut self) -> Option<String> {
        self.trim_left();
        self.skip_commentary();

        match self.stream.chars().next() {
            Some(c) => match c {
                '"' => {
                    let s = Self::string_item(self.stream);
                    self.stream = &self.stream[s.len()..];
                    Some(s)
                }
                c if token::PUNCTUATION.contains(&c) => {
                    self.stream = &self.stream[1..];
                    Some(c.to_string())
                }
                _ => {
                    let str = self
                        .stream
                        .chars()
                        .take_while(|c| !c.is_whitespace() && !PUNCTUATION.contains(&c))
                        .collect::<String>();

                    self.stream = &self.stream[str.len()..];
                    Some(str)
                }
            },
            None => None,
        }
    }

    pub fn next_token(&mut self) -> Option<Result<Token, TokenError>> {
        match self.next_item() {
            Some(s) => Some(Token::from_str(&s)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::Lexer;

    #[test]
    fn test_next_line() {
        let mut lex = Lexer::new("1 linha\n2 linha");
        lex.next_line();
        assert_eq!("2 linha", lex.stream());
        assert_eq!(2, lex.line());
    }

    #[test]
    fn test_trim_left() {
        let mut lex = Lexer::new(" \t\n  2 linha");
        lex.trim_left();
        assert_eq!("2 linha", lex.stream());
        assert_eq!(2, lex.line());
        assert_eq!(3, lex.column());
    }

    #[test]
    fn test_skip_commentary() {
        let mut lex = Lexer::new("; 1\n\t\t ;2\n\t3 linha");
        lex.skip_commentary();
        assert_eq!("3 linha", lex.stream());
        assert_eq!(3, lex.line());
        assert_eq!(2, lex.column());
    }

    #[test]
    fn test_item_identifier_declaration() {
        {
            let mut lex = Lexer::new("main:");
            assert_eq!("main".to_string(), lex.next_item().unwrap());
            assert_eq!(":", lex.stream())
        }

        {
            let mut lex = Lexer::new("foo_1:");
            assert_eq!("foo_1".to_string(), lex.next_item().unwrap());
            assert_eq!(":", lex.stream())
        }
    }

    #[test]
    fn test_keyword() {
        {
            let mut lex = Lexer::new("var position_1");
            assert_eq!("var".to_string(), lex.next_item().unwrap());
            assert_eq!(" position_1", lex.stream());
        }

        {
            let mut lex = Lexer::new("; comentary \n \tvar position_1");
            assert_eq!("var".to_string(), lex.next_item().unwrap());
            assert_eq!(" position_1", lex.stream());
        }
    }

    #[test]
    fn test_token_keyword() {
        let mut lex = Lexer::new("var position_1");
        assert_eq!(
            Token::Keyword(crate::token::Keyword::Var),
            lex.next_token().unwrap().unwrap()
        )
    }

    #[test]
    fn test_token_instruction() {
        let mut lex = Lexer::new("ADD R1, R2, R3");
        assert_eq!(
            Token::Instruction(isa::Instruction::ADD),
            lex.next_token().unwrap().unwrap()
        )
    }

    #[test]
    fn test_token_identifier() {
        let mut lex = Lexer::new("foo_1:");
        assert_eq!(
            Token::Identifier("foo_1".to_string()),
            lex.next_token().unwrap().unwrap()
        )
    }
}

// use crate::token::*;
// use isa::Instruction;
// use std::{char, error::Error, fmt::Display};
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum LexerError {
//     CharBadFormat,
//     StringBadFormat,
//     NumberBadFormat,
//     Empty,
//     InvalidCharacter,
//     InvalidRule,
// }
//
// impl Display for LexerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             LexerError::CharBadFormat => write!(f, "Formato do char inválido!"),
//             LexerError::StringBadFormat => write!(f, "Formato da string inválido!"),
//             LexerError::NumberBadFormat => write!(f, "Formato do número inválido!"),
//             LexerError::Empty => write!(f, "Entrada de dados vazia."),
//             LexerError::InvalidCharacter => write!(f, "Caracter inválido!"),
//             LexerError::InvalidRule => write!(f, "Regra inválida!"),
//         }
//     }
// }
//
// impl Error for LexerError {}
//
// pub struct Lexer<'a> {
//     input: &'a str,
//     stream: Vec<(Token, String)>,
//     curr_pos: usize,
//     curr_line: usize,
//     curr_col: usize,
// }
//
// impl<'a> Lexer<'a> {
//     pub fn new(input: &'a str) -> Self {
//         Self {
//             input,
//             stream: Vec::new(),
//             curr_pos: 0,
//             curr_line: 0,
//             curr_col: 0,
//         }
//     }
//
//     fn inc_line(&mut self) {
//         self.curr_line += 1;
//         self.curr_col = 0;
//     }
//
//     fn skip_next_line(&mut self) {
//         match self.input.split_once(|c| c == '\n') {
//             Some((_, s2)) => {
//                 self.input = s2;
//                 self.inc_line();
//             }
//             None => self.input = "",
//         }
//     }
//
//     fn consume_left_whitespaces(&mut self) {
//         if self.input.starts_with(char::is_whitespace) {
//             for (i, c) in self.input.char_indices() {
//                 match c {
//                     c if c == ' ' || c == '\t' || c == '\r' => continue,
//                     c if c == '\n' => self.inc_line(),
//                     _ => {
//                         self.input = &self.input[i..];
//                         self.curr_col = i;
//                         break;
//                     }
//                 }
//             }
//         }
//     }
//
//     fn consume_commentary(&mut self) {
//         while self.input.starts_with(|c: char| c == ';') {
//             self.skip_next_line();
//             self.consume_left_whitespaces();
//         }
//     }
//
//     fn next_item(&mut self) -> Result<String, LexerError> {
//         self.consume_left_whitespaces();
//         self.consume_commentary();
//
//         macro_rules! surround_by {
//             ($string:expr, $sep:literal) => {
//                 if $string.len() > 0 && $string.chars().next().unwrap() == $sep {
//                     let mut index = 0;
//                     let mut flag = false;
//                     let p = $string.char_indices();
//                     for (i, c) in p {
//                         index = i;
//                         if i != 0 {
//                             match c {
//                                 '\\' => flag = true,
//                                 $sep => match flag {
//                                     true => flag = false,
//                                     false => break,
//                                 },
//                                 _ => flag = false,
//                             }
//                         }
//                     }
//
//                     if index < $string.len() {
//                         Some($string[..index + 1].to_string())
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 }
//             };
//         }
//
//         match self.input.chars().next() {
//             Some(c) => match c {
//                 '\'' => match surround_by!(self.input, '\'') {
//                     Some(s) => {
//                         self.input = &self.input[s.len()..];
//                         Ok(s)
//                     }
//                     None => Err(LexerError::CharBadFormat),
//                 },
//                 '"' => match surround_by!(self.input, '"') {
//                     Some(s) => {
//                         self.input = &self.input[s.len()..];
//                         Ok(s)
//                     }
//                     None => Err(LexerError::StringBadFormat),
//                 },
//                 c if c.is_alphanumeric() => {
//                     let s: String = self
//                         .input
//                         .chars()
//                         .take_while(|c| {
//                             !c.is_whitespace() && (*c == '_' || !c.is_ascii_punctuation())
//                         })
//                         .collect();
//                     self.input = &self.input[s.len()..];
//                     Ok(s)
//                 }
//                 _ => {
//                     let s = self.input.chars().next().unwrap().to_string();
//                     self.input = &self.input[1..];
//                     Ok(s)
//                 }
//             },
//             None => Err(LexerError::Empty),
//         }
//     }
//
//     fn next_number(s: String) -> Result<(Token, String), LexerError> {
//         if usize::from_str_radix(s.as_str(), 10).is_ok() {
//             Ok((Token::Literal(Literal::DecNumber), s))
//         } else if s.chars().next().unwrap() == '0' && s.len() > 2 {
//             match s.chars().nth(1).unwrap() {
//                 c if c == 'b' || c == 'B' => match usize::from_str_radix(&s.as_str()[2..], 2) {
//                     Ok(_) => Ok((
//                         Token::Literal(Literal::BinNumber),
//                         s.as_str()[2..].to_string(),
//                     )),
//                     Err(_) => Err(LexerError::NumberBadFormat),
//                 },
//                 c if c == 'x' || c == 'X' => match usize::from_str_radix(&s.as_str()[2..], 16) {
//                     Ok(_) => Ok((
//                         Token::Literal(Literal::HexNumber),
//                         s.as_str()[2..].to_string(),
//                     )),
//                     Err(_) => Err(LexerError::NumberBadFormat),
//                 },
//                 _ => Err(LexerError::NumberBadFormat),
//             }
//         } else {
//             Err(LexerError::NumberBadFormat)
//         }
//     }
//
//     fn next_word(s: String) -> Result<(Token, String), LexerError> {
//         if let Some(kw) = Keyword::from_str(s.as_str()) {
//             Ok((Token::Keyword(kw), s))
//         } else if let Some(i) = Instruction::from_str(s.as_str()) {
//             Ok((Token::Instruction(i), s))
//         } else {
//             Ok((Token::Identifier, s))
//         }
//     }
//
//     fn next_punctuation(s: String) -> Result<(Token, String), LexerError> {
//         if let Some(p) = Punctuation::from_str(s.as_str()) {
//             Ok((Token::Punctuation(p), s))
//         } else {
//             Err(LexerError::InvalidCharacter)
//         }
//     }
//
//     pub fn next_token(&mut self) -> Result<(Token, String), LexerError> {
//         match self.next_item() {
//             Ok(w) => match w.chars().next().unwrap() {
//                 c if c.is_digit(10) => Lexer::next_number(w),
//                 c if c.is_alphabetic() => Lexer::next_word(w),
//                 c if c == '\'' => Ok((Token::Literal(Literal::Char), w)),
//                 c if c == '"' => Ok((
//                     Token::Literal(Literal::String),
//                     w.as_str()[1..w.len() - 1].to_string(),
//                 )),
//                 _ => Lexer::next_punctuation(w),
//             },
//             Err(e) => Err(e),
//         }
//     }
//
//     fn ruler_check(&mut self) -> Result<(), LexerError> {
//         macro_rules! check_stream {
//             ($(($($next:expr);+)),+) => {{
//                 $(
//                     let t = self.next_token()?;
//                     let mut temp = Vec::new();
//                     match t.0 {
//                         $(token if token == $next => {
//                             self.stream.push(t);
//                             temp.push(stringify!($next).to_string());
//                         })+
//                         _ => return Err(LexerError::InvalidRule),
//                     }
//                 )+
//                 Ok(())
//             }};
//         }
//
//         let token = self.next_token()?;
//         self.stream.push(token.clone()); // Melhorar depois
//
//         match token.0 {
//             Token::Keyword(_) => todo!(),
//             Token::Instruction(i) => match i {
//                 Instruction::InvalidInstruction => unreachable!(),
//
//                 Instruction::LOAD => check_stream!((
//                     Keyword::R0
//                         ; Keyword::R1
//                         ; Keyword::R2
//                         ; Keyword::R3
//                         ; Keyword::R4
//                         ; Keyword::R5
//                         ; Keyword::R6
//                         ; Keyword::R7),
//                     (Punctuation::Comma),
//                     (Literal::DecNumber
//                         ; Literal::BinNumber
//                         ; Literal::HexNumber
//                         ; Token::Identifier)
//                 ),
//
//                 Instruction::LOADN => check_stream!(
//                     (Keyword::R0
//                         ; Keyword::R1
//                         ; Keyword::R2
//                         ; Keyword::R3
//                         ; Keyword::R4
//                         ; Keyword::R5
//                         ; Keyword::R6
//                         ; Keyword::R7),
//                     (Punctuation::Comma),
//                     (Punctuation::Pound),
//                     (Literal::DecNumber
//                         ; Literal::BinNumber
//                         ; Literal::HexNumber
//                         ; Literal::Char
//                         ; Token::Identifier)
//                 ),
//
//                 Instruction::LOADI | Instruction::STOREI | Instruction::CMP => check_stream!(
//                     (Keyword::R0
//                         ; Keyword::R1
//                         ; Keyword::R2
//                         ; Keyword::R3
//                         ; Keyword::R4
//                         ; Keyword::R5
//                         ; Keyword::R6
//                         ; Keyword::R7),
//                     (Punctuation::Comma),
//                     (Keyword::R0
//                         ; Keyword::R1
//                         ; Keyword::R2
//                         ; Keyword::R3
//                         ; Keyword::R4
//                         ; Keyword::R5
//                         ; Keyword::R6
//                         ; Keyword::R7)
//                 ),
//
//                 Instruction::STORE => check_stream!(
//                     (Literal::DecNumber
//                         ; Literal::BinNumber
//                         ; Literal::HexNumber
//                         ; Token::Identifier),
//                     (Punctuation::Comma),
//                     (Keyword::R0
//                         ; Keyword::R1
//                         ; Keyword::R2
//                         ; Keyword::R3
//                         ; Keyword::R4
//                         ; Keyword::R5
//                         ; Keyword::R6
//                         ; Keyword::R7)
//                 ),
//
//                 Instruction::STOREN => check_stream!(
//                     (Literal::DecNumber
//                         ; Literal::BinNumber
//                         ; Literal::HexNumber
//                         ; Token::Identifier),
//                     (Punctuation::Comma),
//                     (Literal::DecNumber
//                         ; Literal::BinNumber
//                         ; Literal::HexNumber
//                         ; Literal::Char
//                         ; Token::Identifier)
//                 ),
//
//                 Instruction::MOV => todo!(),
//                 Instruction::INPUT => todo!(),
//                 Instruction::OUTPUT => todo!(),
//                 Instruction::OUTCHAR => todo!(),
//                 Instruction::INCHAR => todo!(),
//                 Instruction::SOUND => todo!(),
//
//                 Instruction::ADD
//                 | Instruction::ADDC
//                 | Instruction::SUB
//                 | Instruction::SUBC
//                 | Instruction::MUL
//                 | Instruction::DIV
//                 | Instruction::MOD
//                 | Instruction::AND
//                 | Instruction::XOR
//                 | Instruction::OR => {
//                     check_stream!(
//                         (Keyword::R0
//                             ; Keyword::R1
//                             ; Keyword::R2
//                             ; Keyword::R3
//                             ; Keyword::R4
//                             ; Keyword::R5
//                             ; Keyword::R6
//                             ; Keyword::R7),
//                         (Punctuation::Comma),
//                         (Keyword::R0
//                             ; Keyword::R1
//                             ; Keyword::R2
//                             ; Keyword::R3
//                             ; Keyword::R4
//                             ; Keyword::R5
//                             ; Keyword::R6
//                             ; Keyword::R7),
//                         (Punctuation::Comma),
//                         (Keyword::R0
//                             ; Keyword::R1
//                             ; Keyword::R2
//                             ; Keyword::R3
//                             ; Keyword::R4
//                             ; Keyword::R5
//                             ; Keyword::R6
//                             ; Keyword::R7)
//                     )
//                 }
//
//                 Instruction::INC => todo!(),
//                 Instruction::DEC => todo!(),
//                 Instruction::NOT => todo!(),
//                 Instruction::SHIFTL0 => todo!(),
//                 Instruction::SHIFTL1 => todo!(),
//                 Instruction::SHIFTR0 => todo!(),
//                 Instruction::SHIFTR1 => todo!(),
//                 Instruction::ROTL => todo!(),
//                 Instruction::ROTR => todo!(),
//
//                 Instruction::JMP
//                 | Instruction::JEQ
//                 | Instruction::JZ
//                 | Instruction::JC
//                 | Instruction::JN
//                 | Instruction::JNE
//                 | Instruction::JNZ
//                 | Instruction::JNC
//                 | Instruction::JGR
//                 | Instruction::JLE
//                 | Instruction::JEG
//                 | Instruction::JEL
//                 | Instruction::JOV
//                 | Instruction::JNO
//                 | Instruction::JDZ
//                 | Instruction::CZ
//                 | Instruction::CC
//                 | Instruction::CN
//                 | Instruction::CEQ
//                 | Instruction::CNE
//                 | Instruction::CNZ
//                 | Instruction::CNC
//                 | Instruction::CGR
//                 | Instruction::CEG
//                 | Instruction::CEL
//                 | Instruction::COV
//                 | Instruction::CNO
//                 | Instruction::CDZ
//                 | Instruction::CALL
//                 | Instruction::CLE => check_stream!(
//                     (Literal::DecNumber
//                         ; Literal::BinNumber
//                         ; Literal::HexNumber
//                         ; Token::Identifier)
//                 ),
//
//                 Instruction::RTS
//                 | Instruction::RTI
//                 | Instruction::SETC
//                 | Instruction::CLEARC
//                 | Instruction::HALT
//                 | Instruction::NOP
//                 | Instruction::BREAKP => Ok(()),
//
//                 Instruction::PUSH | Instruction::POP => check_stream!(
//                     (Keyword::R0
//                         ; Keyword::R1
//                         ; Keyword::R2
//                         ; Keyword::R3
//                         ; Keyword::R4
//                         ; Keyword::R5
//                         ; Keyword::R6
//                         ; Keyword::R7
//                         ; Keyword::FR)
//                 ),
//             },
//             Token::Identifier => todo!(),
//             Token::Literal(_) => todo!(),
//             Token::Punctuation(_) => todo!(),
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//
//     use super::*;
//
//     #[test]
//     fn test_consume_left_whitespace() {
//         let input = "   \n\r  Test   ";
//         let mut p = Lexer::new(&input);
//         p.consume_left_whitespaces();
//
//         assert_eq!("Test   ", p.input);
//     }
//
//     #[test]
//     fn test_consume_commentary() {
//         let input = " ; Test 123 Test \n
//             Nice";
//         let mut p = Lexer::new(&input);
//         p.consume_commentary();
//
//         assert_eq!("Nice", p.input);
//     }
//
//     #[test]
//     fn test_consume_mult_commentary() {
//         let input = " ; Test 123 Test \n
//             ; Nice test \n
//             MOV ";
//         let mut p = Lexer::new(&input);
//         p.consume_commentary();
//
//         assert_eq!("MOV ", p.input);
//     }
//
//     #[test]
//     fn test_next_string_1() {
//         let input = "\n\tTest";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!("Test".to_string(), p.next_item().unwrap());
//     }
//
//     #[test]
//     fn test_next_string_2() {
//         let input = "\n\tTest 2Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!("Test".to_string(), p.next_item().unwrap());
//     }
//
//     #[test]
//     fn test_next_string_3() {
//         let input = "\n\tTest:3Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!("Test".to_string(), p.next_item().unwrap());
//     }
//
//     #[test]
//     fn test_next_string_4() {
//         let input = "\n\tTest_4Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!("Test_4Test".to_string(), p.next_item().unwrap());
//     }
//
//     #[test]
//     fn test_next_string_5() {
//         let input = "\n\tTest 5Test";
//         let mut p = Lexer::new(&input);
//
//         p.next_item();
//         assert_eq!("5Test".to_string(), p.next_item().unwrap());
//     }
//
//     #[test]
//     fn test_next_string_6() {
//         let input = r#" "Test 6""#;
//         let mut p = Lexer::new(&input);
//
//         assert_eq!("\"Test 6\"".to_string(), p.next_item().unwrap());
//     }
//
//     #[test]
//     fn test_next_token_literal_string_1() {
//         let input = r#"  "Test literal" 123"#;
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Literal(Literal::String), "Test literal".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     #[ignore]
//     fn test_next_token_literal_string_2() {
//         let input = r#"  "Test literal 123"#;
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (
//                 Token::Literal(Literal::String),
//                 "Test literal 123".to_string()
//             ),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_literal_dec() {
//         let input = " 123456 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Literal(Literal::DecNumber), "123456".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_literal_bin() {
//         let input = " 0b01011 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Literal(Literal::BinNumber), "01011".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_literal_hex() {
//         let input = " 0xffc0 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Literal(Literal::HexNumber), "ffc0".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_keyword() {
//         let input = " var   r0 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Keyword(Keyword::Var), "var".to_string()),
//             p.next_token().unwrap()
//         );
//
//         assert_eq!(
//             (Token::Keyword(Keyword::R0), "r0".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_instruction() {
//         let input = " mov   r0 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Instruction(Instruction::MOV), "mov".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_identifier() {
//         let input = " label: mov   r0 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Identifier, "label".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_next_token_punctuation() {
//         let input = "  ,r0   r0 Test";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(
//             (Token::Punctuation(Punctuation::Comma), ",".to_string()),
//             p.next_token().unwrap()
//         );
//     }
//
//     #[test]
//     fn test_ruler_check() {
//         let input = "  LOADN R0, #0123";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(Ok(()), p.ruler_check());
//
//         let input = "  LOADN R0, #label";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(Ok(()), p.ruler_check());
//
//         let input = "  LOADN R0, #'\''";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(Ok(()), p.ruler_check());
//     }
//
//     #[test]
//     fn test_ruler_check_error() {
//         let input = "  LOADN SP, #'\''";
//         let mut p = Lexer::new(&input);
//
//         assert_eq!(Err(LexerError::InvalidRule), p.ruler_check());
//     }
// }
