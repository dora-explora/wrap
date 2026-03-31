#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    JMP,
    SEQ,
    SNE,
    SLT,
    SGT,
    PRN,
    RET,
    NOP
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Specifier {
    N,
    A,
    B,
    C,
    O,
    W,
}

#[derive(Debug)]
pub enum Operand {
    Immediate(usize),
    Direct((usize, Specifier)),
    Indirect((usize, Specifier)),
    Operation(Operation)
}

#[derive(Debug)]
pub struct Operation {
    pub instruction: Instruction,
    pub operands: Vec<Operand>
}
