use std::{fmt::Display, option::Option};

use isa::Instruction;

macro_rules! token_set {
    ($name:ident, $($variant:ident $lit:literal),+$(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            $($variant),+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $($name::$variant => write!(f, "{}", $lit)),+,
                }
            }
        }

        impl $name {
            pub fn from_str(s: &str) -> Option<Self> {
                $(if s.eq_ignore_ascii_case($lit) {
                    return Some($name::$variant)
                })+
                None
            }
        }
    };
}

token_set!(Keyword,
    String  "string",
    Var     "var",
    R0      "R0",
    R1      "R1",
    R2      "R2",
    R3      "R3",
    R4      "R4",
    R5      "R5",
    R6      "R6",
    R7      "R7",
    FR      "FR",
    SP      "SP");

token_set!(Punctuation,
    Comma ",",
    Pound "#");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Char,
    String,
    BinNumber,
    DecNumber,
    HexNumber,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Instruction(Instruction),
    Identifier,
    Literal(Literal),
    Punctuation(Punctuation),
}

impl PartialEq<Keyword> for Token {
    fn eq(&self, other: &Keyword) -> bool {
        match self {
            Token::Keyword(k) => k == other,
            _ => false,
        }
    }
}

impl PartialEq<Instruction> for Token {
    fn eq(&self, other: &Instruction) -> bool {
        match self {
            Token::Instruction(i) => i == other,
            _ => false,
        }
    }
}

impl PartialEq<Literal> for Token {
    fn eq(&self, other: &Literal) -> bool {
        match self {
            Token::Literal(l) => l == other,
            _ => false,
        }
    }
}

impl PartialEq<Punctuation> for Token {
    fn eq(&self, other: &Punctuation) -> bool {
        match self {
            Token::Punctuation(p) => p == other,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_partial_eq() {
        let t = Token::Keyword(Keyword::R0);
        assert_eq!(t, Keyword::R0)
    }
}
