use crate::token::*;
use isa::Instruction;
use std::{char, error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    CharBadFormat,
    StringBadFormat,
    NumberBadFormat,
    Empty,
    InvalidCharacter,
    InvalidRule,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::CharBadFormat => write!(f, "Formato do char inválido!"),
            ParserError::StringBadFormat => write!(f, "Formato da string inválido!"),
            ParserError::NumberBadFormat => write!(f, "Formato do número inválido!"),
            ParserError::Empty => write!(f, "Entrada de dados vazia."),
            ParserError::InvalidCharacter => write!(f, "Caracter inválido!"),
            ParserError::InvalidRule => write!(f, "Regra inválida!"),
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

    fn skip_next_line(&mut self) {
        match self.input.split_once(|c| c == '\n') {
            Some((_, s2)) => {
                self.input = s2;
                self.inc_line();
            }
            None => self.input = "",
        }
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

    fn consume_commentary(&mut self) {
        self.consume_left_whitespaces();
        while self.input.starts_with(|c: char| c == ';') {
            self.skip_next_line();
            self.consume_left_whitespaces();
        }
    }

    fn next_item(&mut self) -> Result<String, ParserError> {
        self.consume_left_whitespaces();

        macro_rules! surround_by {
            ($string:expr, $sep:literal) => {
                if $string.len() > 0 && $string.chars().next().unwrap() == $sep {
                    let mut index = 0;
                    let mut flag = false;
                    let p = $string.char_indices();
                    for (i, c) in p {
                        index = i;
                        if i != 0 {
                            match c {
                                '\\' => flag = true,
                                $sep => match flag {
                                    true => flag = false,
                                    false => break,
                                },
                                _ => flag = false,
                            }
                        }
                    }

                    if index < $string.len() {
                        Some($string[..index + 1].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
        }

        match self.input.chars().next() {
            Some(c) => match c {
                '\'' => match surround_by!(self.input, '\'') {
                    Some(s) => {
                        self.input = &self.input[s.len()..];
                        Ok(s)
                    }
                    None => Err(ParserError::CharBadFormat),
                },
                '"' => match surround_by!(self.input, '"') {
                    Some(s) => {
                        self.input = &self.input[s.len()..];
                        Ok(s)
                    }
                    None => Err(ParserError::StringBadFormat),
                },
                c if c.is_alphanumeric() => {
                    let s: String = self
                        .input
                        .chars()
                        .take_while(|c| {
                            !c.is_whitespace() && (*c == '_' || !c.is_ascii_punctuation())
                        })
                        .collect();
                    self.input = &self.input[s.len()..];
                    Ok(s)
                }
                _ => {
                    let s = self.input.chars().next().unwrap().to_string();
                    self.input = &self.input[1..];
                    Ok(s)
                }
            },
            None => Err(ParserError::Empty),
        }
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
        match self.next_item() {
            Ok(w) => match w.chars().next().unwrap() {
                c if c.is_digit(10) => Parser::next_number(w),
                c if c.is_alphabetic() => Parser::next_word(w),
                c if c == '\'' => Ok((Token::Literal(Literal::Char), w)),
                c if c == '"' => Ok((
                    Token::Literal(Literal::String),
                    w.as_str()[1..w.len() - 1].to_string(),
                )),
                _ => Parser::next_punctuation(w),
            },
            Err(e) => Err(e),
        }
    }

    fn ruler_check(&mut self) -> Result<(), ParserError> {
        macro_rules! check_stream {
            ($(($($next:expr);+)),+) => {{
                $(
                    let t = self.next_token()?;
                    let mut temp = Vec::new();
                    match t.0 {
                        $(token if token == $next => {
                            self.stream.push(t);
                            temp.push(stringify!($next).to_string());
                        })+
                        _ => return Err(ParserError::InvalidRule),
                    }
                )+
                Ok(())
            }};
        }

        let token = self.next_token()?;
        self.stream.push(token.clone()); // Melhorar depois

        match token.0 {
            Token::Keyword(_) => todo!(),
            Token::Instruction(i) => match i {
                Instruction::InvalidInstruction => unreachable!(),

                Instruction::LOAD => check_stream!((
                    Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7),
                    (Punctuation::Comma),
                    (Literal::DecNumber
                        ; Literal::BinNumber
                        ; Literal::HexNumber
                        ; Token::Identifier)
                ),

                Instruction::LOADN => check_stream!(
                    (Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7),
                    (Punctuation::Comma),
                    (Punctuation::Pound),
                    (Literal::DecNumber
                        ; Literal::BinNumber
                        ; Literal::HexNumber
                        ; Literal::Char
                        ; Token::Identifier)
                ),

                Instruction::LOADI => check_stream!(
                    (Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7),
                    (Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7)
                ),

                Instruction::STORE => check_stream!(
                    (Literal::DecNumber
                        ; Literal::BinNumber
                        ; Literal::HexNumber
                        ; Token::Identifier),
                    (Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7)
                ),

                Instruction::STOREN => check_stream!(
                    (Literal::DecNumber
                        ; Literal::BinNumber
                        ; Literal::HexNumber
                        ; Token::Identifier),
                    (Literal::DecNumber
                        ; Literal::BinNumber
                        ; Literal::HexNumber
                        ; Literal::Char
                        ; Token::Identifier)
                ),

                Instruction::STOREI => check_stream!(
                    (Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7),
                    (Keyword::R0
                        ; Keyword::R1
                        ; Keyword::R2
                        ; Keyword::R3
                        ; Keyword::R4
                        ; Keyword::R5
                        ; Keyword::R6
                        ; Keyword::R7)
                ),

                Instruction::MOV => todo!(),
                Instruction::INPUT => todo!(),
                Instruction::OUTPUT => todo!(),
                Instruction::OUTCHAR => todo!(),
                Instruction::INCHAR => todo!(),
                Instruction::SOUND => todo!(),
                Instruction::ADD => todo!(),
                Instruction::ADDC => todo!(),
                Instruction::SUB => todo!(),
                Instruction::SUBC => todo!(),
                Instruction::MUL => todo!(),
                Instruction::DIV => todo!(),
                Instruction::INC => todo!(),
                Instruction::DEC => todo!(),
                Instruction::MOD => todo!(),
                Instruction::AND => todo!(),
                Instruction::OR => todo!(),
                Instruction::XOR => todo!(),
                Instruction::NOT => todo!(),
                Instruction::SHIFTL0 => todo!(),
                Instruction::SHIFTL1 => todo!(),
                Instruction::SHIFTR0 => todo!(),
                Instruction::SHIFTR1 => todo!(),
                Instruction::ROTL => todo!(),
                Instruction::ROTR => todo!(),
                Instruction::CMP => todo!(),
                Instruction::JMP => todo!(),
                Instruction::JEQ => todo!(),
                Instruction::JNE => todo!(),
                Instruction::JZ => todo!(),
                Instruction::JNZ => todo!(),
                Instruction::JC => todo!(),
                Instruction::JNC => todo!(),
                Instruction::JGR => todo!(),
                Instruction::JLE => todo!(),
                Instruction::JEG => todo!(),
                Instruction::JEL => todo!(),
                Instruction::JOV => todo!(),
                Instruction::JNO => todo!(),
                Instruction::JDZ => todo!(),
                Instruction::JN => todo!(),
                Instruction::CALL => todo!(),
                Instruction::CEQ => todo!(),
                Instruction::CNE => todo!(),
                Instruction::CZ => todo!(),
                Instruction::CNZ => todo!(),
                Instruction::CC => todo!(),
                Instruction::CNC => todo!(),
                Instruction::CGR => todo!(),
                Instruction::CLE => todo!(),
                Instruction::CEG => todo!(),
                Instruction::CEL => todo!(),
                Instruction::COV => todo!(),
                Instruction::CNO => todo!(),
                Instruction::CDZ => todo!(),
                Instruction::CN => todo!(),
                Instruction::RTS => todo!(),
                Instruction::RTI => todo!(),
                Instruction::PUSH => todo!(),
                Instruction::POP => todo!(),
                Instruction::NOP => todo!(),
                Instruction::HALT => todo!(),
                Instruction::CLEARC => todo!(),
                Instruction::SETC => todo!(),
                Instruction::BREAKP => todo!(),
            },
            Token::Identifier => todo!(),
            Token::Literal(_) => todo!(),
            Token::Punctuation(_) => todo!(),
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
    fn test_consume_commentary() {
        let input = " ; Test 123 Test \n
            Nice";
        let mut p = Parser::new(&input);
        p.consume_commentary();

        assert_eq!("Nice", p.input);
    }

    #[test]
    fn test_consume_mult_commentary() {
        let input = " ; Test 123 Test \n
            ; Nice test \n
            MOV ";
        let mut p = Parser::new(&input);
        p.consume_commentary();

        assert_eq!("MOV ", p.input);
    }

    #[test]
    fn test_next_string_1() {
        let input = "\n\tTest";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_item().unwrap());
    }

    #[test]
    fn test_next_string_2() {
        let input = "\n\tTest 2Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_item().unwrap());
    }

    #[test]
    fn test_next_string_3() {
        let input = "\n\tTest:3Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test".to_string(), p.next_item().unwrap());
    }

    #[test]
    fn test_next_string_4() {
        let input = "\n\tTest_4Test";
        let mut p = Parser::new(&input);

        assert_eq!("Test_4Test".to_string(), p.next_item().unwrap());
    }

    #[test]
    fn test_next_string_5() {
        let input = "\n\tTest 5Test";
        let mut p = Parser::new(&input);

        p.next_item();
        assert_eq!("5Test".to_string(), p.next_item().unwrap());
    }

    #[test]
    fn test_next_string_6() {
        let input = r#" "Test 6""#;
        let mut p = Parser::new(&input);

        assert_eq!("\"Test 6\"".to_string(), p.next_item().unwrap());
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
    #[ignore]
    fn test_next_token_literal_string_2() {
        let input = r#"  "Test literal 123"#;
        let mut p = Parser::new(&input);

        assert_eq!(
            (
                Token::Literal(Literal::String),
                "Test literal 123".to_string()
            ),
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

    #[test]
    fn test_ruler_check() {
        let input = "  LOADN R0, #0123";
        let mut p = Parser::new(&input);

        assert_eq!(Ok(()), p.ruler_check());

        let input = "  LOADN R0, #label";
        let mut p = Parser::new(&input);

        assert_eq!(Ok(()), p.ruler_check());

        let input = "  LOADN R0, #'\''";
        let mut p = Parser::new(&input);

        assert_eq!(Ok(()), p.ruler_check());
    }

    #[test]
    fn test_ruler_check_error() {
        let input = "  LOADN SP, #'\''";
        let mut p = Parser::new(&input);

        assert_eq!(Err(ParserError::InvalidRule), p.ruler_check());
    }
}
