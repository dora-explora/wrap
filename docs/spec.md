# WRAP_LANG Specification

## Operation Definition

Every operation consists of 2 or 3 fields: The instruction denoting the function of the operation, and the operands that act as the instruction's arguments.

Each operand can be four different modes: an immediate, direct, indirect, or operation.
 - Immediates are normal numbers, denoted by an equals sign (=val). These are useful in math operations, like finding a value mod 5 with MOD val =5.
 - Directs point to addresses with relative cyclical addressing. For example, JMP 2 will jump two instruction ahead, but if the instruction is at the end of the core then it will jump to the second instruction of the program. Negative direct values point backwards. For math operations or other instructions requiring immediate values, this will resolve to the value of the operand determined by the specifier, regardless of that operand's mode. These are denoted by the lack of a symbol.
 - Indirects resolve to the value of the operand determined by the address and specifier of the current operand, denoted by an underscore (_val). For example, a JMP _1A operation followed by a JMP 2 will jump *3* cells ahead, as the address 2 cells ahead of the second operation is 3 ahead of the first.
 - Operations are exactly what they sound like: whole operations stored as an operand of another operation. These are denoted by brackets (\[operation]).

## Instruction Set
 
| Code | Arguments | Function |
| --- | ------- | - |
| MOV | src dst | Moves (copies) data from the source to the destination |
| ADD | op1 op2 | Adds op1 to op2 and stores the result in the address of op1 |
| SUB | op1 op2 | Subtracts op1 by op2 and stores the result in the address of op1 |
| MUL | op1 op2 | Multiplies op1 with op2 and stores the result in the address of op1 |
| DIV | op1 op2 | Divides op1 by op2 and stores the floored result in the address of op1 |
| MOD | op1 op2 | Takes the remainder of op1 divided by operand2 and stores the result in the address of op1 |
| JMP | address | Jumps execution ahead by the number of cells specified by the address |
| SEQ | op1 op2 | Skip the next instruction if op1 and op2 are equal |
| SNE | op1 op2 | Skip the next instruction if op1 and op2 are not equal |
| SLT | op1 op2 | Skip the next instruction if op1 is less than op2 |
| SGT | op1 op2 | Skip the next instruction if op1 is greater than op2 |
| PRN | op mode | Print the operand to the terminal. a mode of =0 means print as a number, otherwise print as ASCII |
| RET | code {} | End the program with the specified return code |
| NOP | op1 op2 | Does nothing, useful for storing data |
