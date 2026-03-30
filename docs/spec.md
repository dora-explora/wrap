# WRAP_LANG Specification

## Operation Definition

Every operation consists of 2-4 fields: The instruction denoting the function of the operation, and the operands that act as the instruction's arguments.

Each operand can be three different modes: an immediate, direct, indirect, or operation.
 - Immediates are normal numbers, denoted by an equals sign (=val). These are useful in math operations, like finding a value mod 5 with MOD val =5.
 - Directs point to addresses with relative cyclical addressing. For example, JMP 2 will jump two instruction ahead, but if the instruction is at the end of the core then it will jump to the second instruction of the program. Negative direct values point backwards. For math operations or other instructions requiring immediate values, this will resolve to the value of the operand determined by the specifier, regardless of that operands mode. These are denoted by the lack of a symbol.
 - Indirects resolve to the value of the operand determined by the address and specifier of the current operand, denoted by an underscore (_val). For example, a JMP _1 operation followed by a JMP 2 will jump *3* cells ahead, as the address 2 cells ahead of the second operation is 3 ahead of the first.
 - Operations are exactly what they sound like: whole operations stored as an operand of another operation. These are denoted by brackets (\[operation]).

## Instruction Set
 
| Code | Arguments | Function |
| --- | ----------- | - |
| MOV | source dest | Moves (copies) data from the source to the destination |
| ADD | op1 op2 dst | Adds operand 1 to operand 2 and stores the result in the destination |
| SUB | op1 op2 dst | Subtracts operand 1 by operand 2 and stores the result in the destination |
| MUL | op1 op2 dst | Multiplies operand 1 with operand 2 and stores the result in the destination |
| DIV | op1 op2 dst | Divides operand 1 by operand 2 and stores the floored result in the destination |
| MOD | op1 op2 dst | Takes the remainder of operand 1 divided by operand2 and stores the result in the destination |
| JMP | address     | Jumps execution ahead by the number of cells specified by the address* |
| JEQ | op1 op2 adr | Jumps execution ahead by address* if operand 1 and 2 are equal |
| JNE | op1 op2 adr | Jumps execution ahead by address* if operand 1 and 2 are not equal |
| JLT | op1 op2 adr | Jumps execution ahead by address* if operand 1 is less than operand 2 |
| JGT | op1 op2 adr | Jumps execution ahead by address* if operand 1 is greater than operand 2 |
| PRN | op          | Print the operand to the terminal |
| RET | code        | End the program with the specified return code |
| NOP | op1 op2 op3 | Does nothing, useful for storing data |
