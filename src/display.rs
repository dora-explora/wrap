use std::fmt::{Display, Formatter, Result};

use crate::sim::*;
use crate::parser::Token;

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            Instruction::MOV => "MOV",
            Instruction::ADD => "ADD",
            Instruction::SUB => "SUB",
            Instruction::MUL => "MUL",
            Instruction::DIV => "DIV",
            Instruction::MOD => "MOD",
            Instruction::JMP => "JMP",
            Instruction::SEQ => "SEQ",
            Instruction::SNE => "SNE",
            Instruction::SLT => "SLT",
            Instruction::SGT => "SGT",
            Instruction::PRN => "PRN",
            Instruction::RET => "RET",
            Instruction::NOP => "NOP"
        };
        return write!(f, "{}", string);
    }
}

impl Display for Specifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            Specifier::N => "N",
            Specifier::A => "A",
            Specifier::B => "B",
            Specifier::C => "C",
            Specifier::O => "O",
            Specifier::W => "W"
        };
        return write!(f, "{}", string);
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            Operand::Immediate(v) => format!("={v}"),
            Operand::Direct((v, s)) => format!("{v}{s}"),
            Operand::Indirect((v, s)) => format!("_{v}{s}"),
            Operand::Operation(o) => format!("[{o}]"),
        };
        return write!(f, "{}", string);
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut operandstring = String::new();
        for operand in &self.operands {
            operandstring.push_str((operand.to_string() + ", ").as_str());
        }
        let _ = operandstring.split_off(operandstring.len() - 2);
        return write!(f, "{} {}", &self.instruction, operandstring);
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Token::Instruction(i) => write!(f, "Instruction {}", i),
            Token::Value(v) => write!(f, "Value {}", v),
            Token::Specifier(s) => write!(f, "Specifier {}", s),
            Token::Equals => write!(f, "Equals"),
            Token::Underscore => write!(f, "Underscore"),
            Token::OpenBracket => write!(f, "OpenBracket"),
            Token::CloseBracket => write!(f, "CloseBracket"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
        }
    }
}

pub fn str_to_instr(str: &str) -> Option<Instruction> {
    return match str {
        "MOV" => Some(Instruction::MOV),
        "ADD" => Some(Instruction::ADD),
        "SUB" => Some(Instruction::SUB),
        "MUL" => Some(Instruction::MUL),
        "DIV" => Some(Instruction::DIV),
        "MOD" => Some(Instruction::MOD),
        "JMP" => Some(Instruction::JMP),
        "SEQ" => Some(Instruction::SEQ),
        "SNE" => Some(Instruction::SNE),
        "SLT" => Some(Instruction::SLT),
        "SGT" => Some(Instruction::SGT),
        "PRN" => Some(Instruction::PRN),
        "RET" => Some(Instruction::RET),
        "NOP" => Some(Instruction::NOP),
        _ => None
    }
}

pub fn char_to_spec(char: char) -> Option<Specifier> {
    return match char {
        'N' => Some(Specifier::N),
        'A' => Some(Specifier::A),
        'B' => Some(Specifier::B),
        'C' => Some(Specifier::C),
        'O' => Some(Specifier::O),
        'W' => Some(Specifier::W),
        _ => None
    }
}
