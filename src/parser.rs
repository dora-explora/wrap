use std::collections::VecDeque;
use std::result::Result;
use std::fmt::{Display, Formatter, Result as fmtResult};
use std::error::Error;

use crate::sim::*;
use crate::display::{char_to_spec, str_to_instr};

#[derive(Debug, Clone)]
pub struct ParseError { pub message: String, pub line: usize }
impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        ratatui::restore(); // this is a kinda bad hack
        write!(f, "Parse error on line #{}: {}", self.line, self.message)
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
    Semicolon,
    Newline
}

fn tokenize(string: String) -> VecDeque<Token> {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let mut currentinstruction: Vec<char> = Vec::new();
    let mut currentvalue: Vec<usize> = Vec::new();
    let mut comment: bool = false;
    for char in string.chars() {
        if comment {
            if char == '\n' { comment = false; tokens.push_back(Token::Newline); }
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
            '\n' => tokens.push_back(Token::Newline),
            _ => {}
        };
    }
    return tokens;
}

pub fn parse(string: String) -> ParseResult<Vec<Operation>> {
    let mut operations: Vec<Operation> = Vec::new();
    let mut tokens: VecDeque<Token> = tokenize(string);
    let mut line: usize = 1;
    // for token in &tokens {
        // match token {
        //     Token::Instruction(i) => println!("Instruction {i}") ,
        //     Token::Value(v) => println!("Value {v}"),
        //     Token::Specifier(s) => println!("Specifier {s}"),
        //     Token::Equals => println!("Equals"),
        //     Token::Underscore => println!("Underscore"),
        //     Token::OpenBracket => println!("OpenBracket"),
        //     Token::CloseBracket => println!("CloseBracket"),
        //     Token::Comma => println!("Comma"),
        //     Token::Semicolon => println!("Semicolon"),
        //     Token::Newline => println!("Newline"),
        // }
    // }
    while tokens.len() != 0 {
        operations.push(parse_operation(&mut tokens, &mut line)?);
    }
    return Ok(operations);
}

fn parse_operation(tokens: &mut VecDeque<Token>, line: &mut usize) -> ParseResult<Operation> {
    loop { match tokens[0] { Token::Newline => { *line += 1; let _ = tokens.pop_front();}, _ => break } } // filter newlines
    let instr = match tokens[0] {
        Token::Instruction(i) => i,
        _ => return Err(ParseError{line: *line, message: format!("Expected instruction token, found {}", tokens[0])})
    };
    let _ = tokens.pop_front();
    let op_one: Operand = parse_operand(tokens, line, true)?;
    let op_two: Operand = parse_operand(tokens, line, false)?;
    return Ok(Operation {instruction: instr, operands: vec![op_one, op_two] });
}

fn parse_operand(tokens: &mut VecDeque<Token>, line: &mut usize, op_one: bool) -> ParseResult<Operand> {
    let operand: Operand;
    loop { match tokens[0] { Token::Newline => { *line += 1; let _ = tokens.pop_front();}, _ => break } } // filter newlines
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
                _ => return Err(ParseError{line: *line, message: format!("Unexpected token after {}: {}", tokens[0], tokens[1])})
            }
        },
        Token::Equals | Token::Underscore => { // if first token is one of these, then its immediate or indirect
            let value: usize;
            match tokens[1] { // find value where its expected
                Token::Value(v) => {value = v},
                _ => return Err(ParseError{line: *line, message: format!("Unexpected token after {}: {}", tokens[0], tokens[1])})
            }
            let mut specop: Option<Specifier> = None;
            match tokens[2] { // check if specifier is specified (unecessary if its immediate, hence the Option)
                Token::Specifier(s) => specop = Some(s),
                Token::Comma | Token::Semicolon | Token::CloseBracket => {}
                _ => return Err(ParseError{line: *line, message: format!("Cannot end operand with {}", tokens[2])})
            }
            if tokens[0] == Token::Equals { operand = Operand::Immediate(value); } // if it was an '=', return immediate
            else if specop != None { operand = Operand::Indirect((value, specop.unwrap())); } // otherwise, should be indirect: check for specifier
            else { return Err(ParseError{line: *line, message: format!("Indirect of value {value} needs specifier")}); } // no specifier? error
            let _ = tokens.pop_front();
            let _ = tokens.pop_front();
            let _ = tokens.pop_front();
        },
        Token::OpenBracket => { // if its an operation, run parse_operation recursively
            let _ = tokens.pop_front();
            operand = Operand::Operation(parse_operation(tokens, line)?);
        },
        _ => return Err(ParseError{line: *line, message: format!("Cannot start operand with {}", tokens[0])})
    }
    if op_one {
        match &tokens[0] {
            Token::Semicolon | Token::CloseBracket =>
                return Err(ParseError{line: *line, message: "Cannot end operand one with semicolon or closed bracket".to_string()}),
            Token::Comma => { let _ = tokens.pop_front(); },
            t =>
                return Err(ParseError{line: *line, message: format!("Cannot end operand one with {}", t)}),
        }
    } else {
        match &tokens[0] {
            Token::Comma =>
                return Err(ParseError{line: *line, message: "Cannot end operand two with comma".to_string()}),
            Token::Semicolon | Token::CloseBracket => { let _ = tokens.pop_front(); },
            t =>
                return Err(ParseError{line: *line, message: format!("Cannot end operand one with {}", t)}),
        }
    }
    loop { match tokens.get(0).unwrap_or(&Token::Equals) { Token::Newline => { *line += 1; let _ = tokens.pop_front();}, _ => break } } // filter newlines
    return Ok(operand);
}
