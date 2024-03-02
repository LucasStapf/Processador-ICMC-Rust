#![allow(dead_code, unused_imports)]

use std::{collections::HashMap, fs::File};

use isa::Instruction;
use lexer::Lexer;
use thiserror::Error;
use token::{Token, TokenError};

use crate::token::Keyword;

mod lexer;
mod token;

#[derive(Error, Debug, PartialEq)]
pub enum AssemblerError {
    #[error("Esperado {expected:?}, Recebido: {received:?}")]
    UnexpectedToken {
        expected: Option<Token>,
        received: Option<Token>,
    },

    #[error("{0}")]
    InvalidToken(TokenError),
}

pub struct Assembler<'a> {
    current_address: usize,
    labels: HashMap<String, usize>,
    lex: Lexer<'a>,
    stream_in: String,
    stream_out: Vec<usize>,
}

impl Assembler<'_> {
    fn expected_token<T>(&mut self) -> Result<T, AssemblerError> {
        todo!()
    }

    fn write_instruction(&mut self, instruction: Instruction) -> Result<(), AssemblerError> {
        match instruction {
            Instruction::InvalidInstruction => todo!(),
            Instruction::LOAD => {
                // let register = self.expected_token::<Keyword>()?;
                todo!()
            }
            Instruction::LOADN => todo!(),
            Instruction::LOADI => todo!(),
            Instruction::STORE => todo!(),
            Instruction::STOREN => todo!(),
            Instruction::STOREI => todo!(),
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
        }
    }
}
