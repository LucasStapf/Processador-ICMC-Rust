use std::{error, fmt::Display, option::Option, str::FromStr};

use isa::Instruction;
use thiserror::Error;

pub const KEYWORDS: [&str; 13] = [
    "string", "var", "static", "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "SP", "FR",
];

pub const PUNCTUATION: [char; 4] = [',', '#', ':', '+'];

#[derive(Error, Debug, PartialEq)]
pub enum TokenError {
    #[error("A string \"{0}\" está mal formatada.")]
    StringBadFormat(String),

    #[error("O char \"{0}\" está mal formatado.")]
    CharBadFormat(String),

    #[error("O número \"{0}\" não está em um formato válido.")]
    NumberBadFormat(String),

    #[error("Pontuação \"{0}\" inválida.")]
    InvalidPunctuation(String),

    #[error("Token inválido: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    SP,
    FR,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R0 => write!(f, "R0"),
            Self::R1 => write!(f, "R1"),
            Self::R2 => write!(f, "R2"),
            Self::R3 => write!(f, "R3"),
            Self::R4 => write!(f, "R4"),
            Self::R5 => write!(f, "R5"),
            Self::R6 => write!(f, "R6"),
            Self::R7 => write!(f, "R7"),
            Self::SP => write!(f, "SP"),
            Self::FR => write!(f, "FR"),
        }
    }
}

impl FromStr for Register {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case(&Self::R0.to_string()) => Ok(Self::R0),
            s if s.eq_ignore_ascii_case(&Self::R1.to_string()) => Ok(Self::R1),
            s if s.eq_ignore_ascii_case(&Self::R2.to_string()) => Ok(Self::R2),
            s if s.eq_ignore_ascii_case(&Self::R3.to_string()) => Ok(Self::R3),
            s if s.eq_ignore_ascii_case(&Self::R4.to_string()) => Ok(Self::R4),
            s if s.eq_ignore_ascii_case(&Self::R5.to_string()) => Ok(Self::R5),
            s if s.eq_ignore_ascii_case(&Self::R6.to_string()) => Ok(Self::R6),
            s if s.eq_ignore_ascii_case(&Self::R7.to_string()) => Ok(Self::R7),
            s if s.eq_ignore_ascii_case(&Self::SP.to_string()) => Ok(Self::SP),
            s if s.eq_ignore_ascii_case(&Self::FR.to_string()) => Ok(Self::FR),
            _ => Err(Self::Err::Invalid(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    String,
    Static,
    Var,
    Register(Register),
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::String => write!(f, "string"),
            Keyword::Static => write!(f, "static"),
            Keyword::Var => write!(f, "var"),
            Keyword::Register(r) => write!(f, "{r}"),
        }
    }
}

impl FromStr for Keyword {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case(&Self::String.to_string()) => Ok(Self::String),
            s if s.eq_ignore_ascii_case(&Self::Static.to_string()) => Ok(Self::Static),
            s if s.eq_ignore_ascii_case(&Self::Var.to_string()) => Ok(Self::Var),
            s if Register::from_str(s).is_ok() => {
                Ok(Self::Register(Register::from_str(s).unwrap()))
            }
            _ => Err(Self::Err::Invalid(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    Pound,
    Comma,
    Colon,
    Plus,
}

impl Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Punctuation::Pound => write!(f, "#"),
            Punctuation::Comma => write!(f, ","),
            Punctuation::Colon => write!(f, ":"),
            Punctuation::Plus => write!(f, "+"),
        }
    }
}

impl FromStr for Punctuation {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s == Self::Pound.to_string() => Ok(Self::Pound),
            s if s == Self::Comma.to_string() => Ok(Self::Comma),
            s if s == Self::Colon.to_string() => Ok(Self::Colon),
            s if s == Self::Plus.to_string() => Ok(Self::Plus),
            _ => Err(Self::Err::InvalidPunctuation(s.to_string())),
        }
    }
}

pub struct TokenValue<T> {
    token: Token,
    value: T,
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Instruction(Instruction),
    Identifier(String),
    Number(usize),
    String(String),
    Char(char),
    Punctuation(Punctuation),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Keyword(l0), Self::Keyword(r0)) => l0 == r0,
            (Self::Instruction(l0), Self::Instruction(r0)) => l0 == r0,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Punctuation(l0), Self::Punctuation(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Token {
    fn word(s: &str) -> Result<Self, TokenError> {
        if s.len() == 0 {
            Err(TokenError::Invalid("Tamanho nulo.".to_string()))
        } else if let Ok(kw) = Keyword::from_str(s) {
            Ok(Self::Keyword(kw))
        } else if let Some(i) = Instruction::from_str(s) {
            Ok(Self::Instruction(i))
        } else {
            Ok(Self::Identifier(s.to_string()))
        }
    }

    fn number(s: &str) -> Result<Self, TokenError> {
        let re = regex::RegexSet::new(&[
            r"^[[:digit:]]{1,6}$",
            r"^0[xX][[:xdigit:]]{1,4}$",
            r"^0[bB][[0-1]{1,16}$]",
        ])
        .unwrap();

        match re.is_match(s) {
            true => {
                let ret_val = if let Ok(n) = usize::from_str_radix(&s, 10) {
                    n
                } else if let Ok(n) = usize::from_str_radix(&s[2..], 2) {
                    n
                } else if let Ok(n) = usize::from_str_radix(&s[2..], 16) {
                    n
                } else {
                    return Err(TokenError::NumberBadFormat(s.to_string()));
                };

                Ok(Self::Number(ret_val))
            }
            false => Err(TokenError::NumberBadFormat(s.to_string())),
        }
    }

    fn string(s: &str) -> Result<Self, TokenError> {
        if s.starts_with('"') && s.ends_with('"') {
            let str = &s[1..s.len() - 1];
            let mut iter = str.chars();
            let mut ret_val = String::with_capacity(str.len());

            while let Some(c) = iter.next() {
                match c {
                    '\\' => match iter.next() {
                        Some(c) => match c {
                            'r' => ret_val.push('\r'),
                            'n' => ret_val.push('\n'),
                            't' => ret_val.push('\t'),
                            '\'' => ret_val.push('\''),
                            '"' => ret_val.push('"'),
                            '\\' => ret_val.push('\\'),
                            _ => return Err(TokenError::StringBadFormat(s.to_string())),
                        },
                        None => return Err(TokenError::StringBadFormat(s.to_string())),
                    },
                    _ => ret_val.push(c),
                }
            }

            Ok(Token::String(ret_val))
        } else {
            Err(TokenError::StringBadFormat(s.to_string()))
        }
    }

    fn char(s: &str) -> Result<Self, TokenError> {
        if s.starts_with('\'') && s.ends_with('\'') {
            // '123'
            let c = &s[1..s.len() - 1];

            if c.len() >= 1 && c.len() <= 2 {
                let mut iter = c.chars();
                match iter.next() {
                    Some(ch) => match ch {
                        '\\' => match iter.next() {
                            Some(ch) => match ch {
                                '\'' => Ok(Token::Char('\'')),
                                'n' => Ok(Token::Char('\n')),
                                'r' => Ok(Token::Char('\r')),
                                't' => Ok(Token::Char('\t')),
                                '\\' => Ok(Token::Char('\\')),
                                _ => Err(TokenError::CharBadFormat(s.to_string())),
                            },
                            None => Err(TokenError::CharBadFormat(s.to_string())),
                        },
                        _ => Ok(Token::Char(ch)),
                    },
                    None => Err(TokenError::CharBadFormat(s.to_string())),
                }
            } else {
                Err(TokenError::CharBadFormat(s.to_string()))
            }
        } else {
            Err(TokenError::CharBadFormat(s.to_string()))
        }
    }

    fn punctuation(s: &str) -> Result<Self, TokenError> {
        if s.len() == 1 {
            match Punctuation::from_str(s) {
                Ok(p) => Ok(Token::Punctuation(p)),
                Err(_) => Err(TokenError::InvalidPunctuation(s.to_string())),
            }
        } else {
            Err(TokenError::InvalidPunctuation(s.to_string()))
        }
    }
}

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            Err(TokenError::Invalid("Tamanho nulo.".to_string()))
        } else {
            match s.chars().next().unwrap() {
                c if c.is_alphabetic() => Token::word(&s),
                c if c.is_numeric() => Token::number(&s),
                c if c == '"' => Token::string(&s),
                c if c == '\'' => Token::char(&s),
                c if PUNCTUATION.contains(&c) => Token::punctuation(&s),
                _ => Err(TokenError::Invalid(s.to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Keyword, Punctuation, Token};
    use std::str::FromStr;

    #[test]
    fn test_keyword() {
        {
            let s = "var";
            assert_eq!(Token::Keyword(Keyword::Var), Token::from_str(s).unwrap())
        }

        {
            let s = "VAR";
            assert_eq!(Token::Keyword(Keyword::Var), Token::from_str(s).unwrap())
        }
    }

    #[test]
    fn test_instruction() {
        {
            let s = "push";
            assert_eq!(
                Token::Instruction(isa::Instruction::from_str(s).unwrap()),
                Token::from_str(s).unwrap()
            )
        }

        {
            let s = "PUSH";
            assert_eq!(
                Token::Instruction(isa::Instruction::from_str(s).unwrap()),
                Token::from_str(s).unwrap()
            )
        }
    }

    #[test]
    fn test_identifier() {
        {
            let s = "main";
            assert_eq!(
                Token::Identifier("main".to_string()),
                Token::from_str(s).unwrap()
            )
        }

        {
            let s = "foo_1";
            assert_eq!(
                Token::Identifier("foo_1".to_string()),
                Token::from_str(s).unwrap()
            )
        }
    }

    #[test]
    fn test_number() {
        {
            let s = "2";
            assert_eq!(Token::Number(2), Token::from_str(s).unwrap())
        }

        {
            let s = "0b10";
            assert_eq!(Token::Number(2), Token::from_str(s).unwrap())
        }

        {
            let s = "0x10";
            assert_eq!(Token::Number(2), Token::from_str(s).unwrap())
        }
    }

    #[test]
    fn test_string() {
        {
            let s = r#""test""#;
            assert_eq!(
                Token::String("test".to_string()),
                Token::from_str(s).unwrap()
            )
        }

        {
            let s = "\"test \\\"123\\\".\"";
            assert_eq!(
                Token::String("test \"123\".".to_string()),
                Token::from_str(s).unwrap()
            )
        }
    }

    #[test]
    fn test_char() {
        {
            let s = "'t'";
            assert_eq!(Token::Char('t'), Token::from_str(s).unwrap())
        }

        {
            let s = "'\\''";
            assert_eq!(Token::Char('\''), Token::from_str(s).unwrap())
        }
    }

    #[test]
    fn test_punctuation() {
        {
            let s = "#";
            assert_eq!(
                Token::Punctuation(Punctuation::Pound),
                Token::from_str(s).unwrap()
            )
        }

        {
            let s = ",";
            assert_eq!(
                Token::Punctuation(Punctuation::Comma),
                Token::from_str(s).unwrap()
            )
        }
    }
}
// macro_rules! token_set {
//     ($name:ident, $($variant:ident $lit:literal),+$(,)?) => {
//         #[derive(Clone, Copy, Debug, PartialEq)]
//         pub enum $name {
//             $($variant),+
//         }
//
//         impl std::fmt::Display for $name {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//                 match self {
//                     $($name::$variant => write!(f, "{}", $lit)),+,
//                 }
//             }
//         }
//
//         impl $name {
//             pub fn from_str(s: &str) -> Option<Self> {
//                 $(if s.eq_ignore_ascii_case($lit) {
//                     return Some($name::$variant)
//                 })+
//                 None
//             }
//         }
//     };
// }
//
// token_set!(Keyword,
//     String  "string",
//     Var     "var",
//     R0      "R0",
//     R1      "R1",
//     R2      "R2",
//     R3      "R3",
//     R4      "R4",
//     R5      "R5",
//     R6      "R6",
//     R7      "R7",
//     FR      "FR",
//     SP      "SP");
//
// token_set!(Punctuation,
//     Comma ",",
//     Pound "#");
//
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Literal {
//     Char,
//     String,
//     BinNumber,
//     DecNumber,
//     HexNumber,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Token {
//     Keyword(Keyword),
//     Instruction(Instruction),
//     Identifier,
//     Literal(Literal),
//     Punctuation(Punctuation),
// }
//
// impl PartialEq<Keyword> for Token {
//     fn eq(&self, other: &Keyword) -> bool {
//         match self {
//             Token::Keyword(k) => k == other,
//             _ => false,
//         }
//     }
// }
//
// impl PartialEq<Instruction> for Token {
//     fn eq(&self, other: &Instruction) -> bool {
//         match self {
//             Token::Instruction(i) => i == other,
//             _ => false,
//         }
//     }
// }
//
// impl PartialEq<Literal> for Token {
//     fn eq(&self, other: &Literal) -> bool {
//         match self {
//             Token::Literal(l) => l == other,
//             _ => false,
//         }
//     }
// }
//
// impl PartialEq<Punctuation> for Token {
//     fn eq(&self, other: &Punctuation) -> bool {
//         match self {
//             Token::Punctuation(p) => p == other,
//             _ => false,
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_tokens_partial_eq() {
//         let t = Token::Keyword(Keyword::R0);
//         assert_eq!(t, Keyword::R0)
//     }
// }
