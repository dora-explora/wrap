use std::collections::VecDeque;
use std::result::Result;
    use std::fmt::{Display, Formatter, Result as fmtResult};

use crate::sim::*;
use crate::display::{char_to_spec, str_to_instr};

#[derive(Debug, Clone)]
pub struct ParseError { pub message: String }

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "Parse error: {}", self.message)
    }
}

type ParseResult<T> = Result<T, ParseError>;

#[derive(PartialEq)]
pub enum Token {
    Instruction(Instruction),
    Value(usize),
    Specifier(Specifier),
    Equals,
    Underscore,
    OpenBracket,
    CloseBracket,
    Comma,
    Semicolon
}

fn tokenize(string: String) -> VecDeque<Token> {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let mut currentinstruction: Vec<char> = Vec::new();
    let mut currentvalue: Vec<usize> = Vec::new();
    let mut comment: bool = false;
    for char in string.chars() {
        if comment {
            if char == '\n' { comment = false; }
            continue;
        }
        if char == '#' { comment = true; }
        if char.is_alphabetic() { // instruction and specifier matching
            currentinstruction.push(char);
            if currentinstruction.len() == 3 {
                let string = format!("{}{}{}", currentinstruction[0], currentinstruction[1], currentinstruction[2]);
                match str_to_instr(string.as_str()) {
                    Some(instr) => tokens.push_back(Token::Instruction(instr)),
                    None => {}
                }
                currentinstruction.clear();
            }
        } else if currentinstruction.len() == 1 {
            match char_to_spec(currentinstruction[0]) {
                Some(spec) => tokens.push_back(Token::Specifier(spec)),
                None => {}
            }
            currentinstruction.clear();
        }

        if char.is_numeric() { // value matching
            currentvalue.push(char.to_string().parse::<usize>().unwrap());
        } else if currentvalue.len() != 0 {
            let mut value = 0;
            for digit in &currentvalue {
                value *= 10;
                value += digit;
            }
            tokens.push_back(Token::Value(value));
            currentvalue.clear();
        }

        match char { // special character matching
            '=' => tokens.push_back(Token::Equals),
            '_' => tokens.push_back(Token::Underscore),
            '[' => tokens.push_back(Token::OpenBracket),
            ';' => tokens.push_back(Token::Semicolon),
            ']' => tokens.push_back(Token::CloseBracket),
            ',' => tokens.push_back(Token::Comma),
            _ => {}
        };
    }
    return tokens;
}

pub fn parse(string: String) -> ParseResult<Vec<Operation>> {
    let mut operations: Vec<Operation> = Vec::new();
    let mut tokens: VecDeque<Token> = tokenize(string);
    for token in &tokens {
        match token {
            Token::Instruction(i) => println!("Instruction {i}") ,
            Token::Value(v) => println!("Value {v}"),
            Token::Specifier(s) => println!("Specifier {s}"),
            Token::Equals => println!("Equals"),
            Token::Underscore => println!("Underscore"),
            Token::OpenBracket => println!("OpenBracket"),
            Token::Semicolon => println!("Semicolon"),
            Token::CloseBracket => println!("CloseBracket"),
            Token::Comma => println!("Comma"),
        }
    }
    while tokens.len() != 0 {
        operations.push(parse_operation(&mut tokens)?);
    }
    return Ok(operations);
}

fn parse_operation(tokens: &mut VecDeque<Token>) -> ParseResult<Operation> {
    let instr = match tokens[0] {
        Token::Instruction(i) => i,
        _ => return Err(ParseError{message: format!("Expected instruction token, found {}", tokens[0])})
    };
    let _ = tokens.pop_front();
    let op_one: Operand = parse_operand(tokens, true)?;
    println!("op one: {}", op_one);
    let op_two: Operand = parse_operand(tokens, false)?;
    println!("op two: {}", op_two);
    return Ok(Operation {instruction: instr, operands: vec![op_one, op_two] });
}

fn parse_operand(tokens: &mut VecDeque<Token>, op_one: bool) -> ParseResult<Operand> {
    let operand: Operand;
    match tokens[0] { // good luck reading this
        Token::Value(v) => { // if first token is a value, its a direct
           match tokens[1] {
               Token::Specifier(s) => { // if specifier is specified, then just return
                   let _ = tokens.pop_front();
                   let _ = tokens.pop_front();
                   operand = Operand::Direct((v, s));
               },
               Token::Comma | Token::Semicolon | Token::CloseBracket => { // otherwise make sure syntax is correct then return N
                       let _ = tokens.pop_front();
                       operand = Operand::Direct((v, Specifier::N));
               },
               _ => return Err(ParseError{message: format!("Unexpected token after {}: {}", tokens[0], tokens[1])})
           }
        },
        Token::Equals | Token::Underscore => { // if first token is one of these, then its immediate or indirect
            let value: usize;
            match tokens[1] { // find value where its expected
                Token::Value(v) => {value = v},
                _ => return Err(ParseError{message: format!("Unexpected token after {}: {}", tokens[0], tokens[1])})
            }
            let mut specop: Option<Specifier> = None;
            match tokens[2] { // check if specifier is specified (unecessary if its immediate, hence the Option)
                Token::Specifier(s) => specop = Some(s),
                Token::Comma | Token::Semicolon | Token::CloseBracket => {}
                _ => return Err(ParseError{message: format!("Cannot end operand with {}", tokens[2])})
            }
            if tokens[0] == Token::Equals { operand = Operand::Immediate(value); } // if it was an '=', return immediate
            else if specop != None { operand = Operand::Indirect((value, specop.unwrap())); } // otherwise, should be indirect: check for specifier
            else { return Err(ParseError{message: format!("Indirect of value {value} needs specifier")}); } // no specifier? error
            let _ = tokens.pop_front();
            let _ = tokens.pop_front();
            let _ = tokens.pop_front();
        },
        Token::OpenBracket => { // if its an operation, run parse_operation recursively
            let _ = tokens.pop_front();
            operand = Operand::Operation(parse_operation(tokens)?);
        },
        _ => return Err(ParseError{message: format!("Cannot start operand with {}", tokens[0])})
    }
    if tokens[0] == Token::Comma && !op_one {
        return Err(ParseError{message: "Cannot end operand two with comma".to_string()});
    } else if (tokens[0] == Token::Semicolon || tokens[0] == Token::CloseBracket) && op_one {
        return Err(ParseError{message: "Cannot end operand one with semicolon or closed bracket".to_string()});
    }
    let _ = tokens.pop_front();
    return Ok(operand);
}
