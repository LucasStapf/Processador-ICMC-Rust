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

pub enum Literal {
    String,
    BinNumber,
    DecNumber,
    HexNumber,
}

pub enum Token {
    Keyword(Keyword),
    Instruction(Instruction),
    Identifier,
    Literal(Literal),
    Punctuation(Punctuation),
}
