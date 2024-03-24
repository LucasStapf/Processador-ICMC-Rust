use std::{any::Any, error, fmt::Display, option::Option, str::FromStr};

use isa::Instruction;
use thiserror::Error;

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

pub trait TokenType {
    fn same_type(&self, other: Self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Instruction(Instruction),
    Identifier(String),
    Number(usize),
    LiteralChar(char),
    LiteralStr(String),
    String,
    Static,
    Var,
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
    Pound,
    Comma,
    Colon,
    Plus,
}

impl TokenType for Token {
    fn same_type(&self, other: Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instruction(i) => write!(f, "{i}"),
            Self::Identifier(id) => write!(f, "{id}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::LiteralChar(c) => write!(f, "'{c}'"),
            Self::LiteralStr(s) => write!(f, "\"{s}\""),
            Self::String => write!(f, "string"),
            Self::Static => write!(f, "static"),
            Self::Var => write!(f, "var"),
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
            Self::Pound => write!(f, "#"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::Plus => write!(f, "+"),
        }
    }
}

impl Token {
    fn word(s: &str) -> Result<Self, TokenError> {
        if s.len() == 0 {
            Err(TokenError::Invalid("Tamanho nulo.".to_string()))
        } else if let Ok(inst) = Instruction::from_str(s) {
            Ok(Self::Instruction(inst))
        } else {
            match s {
                s if s.eq_ignore_ascii_case(&Self::String.to_string()) => Ok(Self::String),
                s if s.eq_ignore_ascii_case(&Self::Var.to_string()) => Ok(Self::Var),
                s if s.eq_ignore_ascii_case(&Self::Static.to_string()) => Ok(Self::Static),
                _ => Ok(Self::Identifier(s.to_string())),
            }
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

            Ok(Token::LiteralStr(ret_val))
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
                                '\'' => Ok(Token::LiteralChar('\'')),
                                'n' => Ok(Token::LiteralChar('\n')),
                                'r' => Ok(Token::LiteralChar('\r')),
                                't' => Ok(Token::LiteralChar('\t')),
                                '\\' => Ok(Token::LiteralChar('\\')),
                                _ => Err(TokenError::CharBadFormat(s.to_string())),
                            },
                            None => Err(TokenError::CharBadFormat(s.to_string())),
                        },
                        _ => Ok(Token::LiteralChar(ch)),
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
            match s {
                s if Self::Comma.to_string().eq(s) => Ok(Self::Comma),
                s if Self::Colon.to_string().eq(s) => Ok(Self::Colon),
                s if Self::Plus.to_string().eq(s) => Ok(Self::Plus),
                s if Self::Pound.to_string().eq(s) => Ok(Self::Pound),
                _ => Err(TokenError::InvalidPunctuation(s.to_string())),
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
                c if c.is_alphabetic() => Self::word(&s),
                c if c.is_numeric() => Self::number(&s),
                c if c == '"' => Self::string(&s),
                c if c == '\'' => Self::char(&s),
                c if c == '+' || c == '#' || c == ',' || c == ':' => Self::punctuation(s),
                _ => Err(TokenError::Invalid(s.to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use std::str::FromStr;

    #[test]
    fn test_keyword() {
        {
            let s = "var";
            assert_eq!(Token::Var, Token::from_str(s).unwrap())
        }

        {
            let s = "VAR";
            assert_eq!(Token::Var, Token::from_str(s).unwrap())
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
                Token::LiteralStr("test".to_string()),
                Token::from_str(s).unwrap()
            )
        }

        {
            let s = "\"test \\\"123\\\".\"";
            assert_eq!(
                Token::LiteralStr("test \"123\".".to_string()),
                Token::from_str(s).unwrap()
            )
        }
    }

    #[test]
    fn test_char() {
        {
            let s = "'t'";
            assert_eq!(Token::LiteralChar('t'), Token::from_str(s).unwrap())
        }

        {
            let s = "'\\''";
            assert_eq!(Token::LiteralChar('\''), Token::from_str(s).unwrap())
        }
    }

    #[test]
    fn test_punctuation() {
        {
            let s = "#";
            assert_eq!(Token::Pound, Token::from_str(s).unwrap())
        }

        {
            let s = ",";
            assert_eq!(Token::Comma, Token::from_str(s).unwrap())
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
