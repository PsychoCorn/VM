use super::Word;

pub struct OpCode;

impl OpCode {
    #[allow(dead_code)]
    pub const CODE_MASK: Word = 0b000011111111;
    #[allow(dead_code)]
    pub const TYPE_MASK: Word = 0b111000000000;

    // stack operations, operand in cx
    pub const PUSH         : Word =  1;
    pub const POP          : Word =  2;

    // arithmetic and bitwise operations
    // prefix f - for real numbers
    // @ - operator
    // unary: ax = @ax
    // binary: ax = ax @ bx
    // div: ax - quot, dx - rem
    pub const INC          : Word =  3;
    pub const DEC          : Word =  4;
    pub const NEG          : Word =  5;
    pub const ADD          : Word =  6;
    pub const SUB          : Word =  7;
    pub const MUL          : Word =  8;
    pub const DIV          : Word =  9;
    pub const FNEG         : Word = 10;
    pub const FADD         : Word = 11;
    pub const FSUB         : Word = 12;
    pub const FMUL         : Word = 13;
    pub const FDIV         : Word = 14;
    pub const AND          : Word = 15;
    pub const OR           : Word = 16;
    pub const XOR          : Word = 17;
    pub const NOT          : Word = 18;
    pub const SHL          : Word = 19;
    pub const SHR          : Word = 20;
    
    // comparations and jumps
    // ax - left operand
    // bx - right operand
    // dx - address

    // universal jumps
    pub const JMP          : Word = 21;
    pub const JE           : Word = 22;
    pub const JNE          : Word = 23;
    // for sword
    pub const JG           : Word = 24;
    pub const JGE          : Word = 25;
    pub const JL           : Word = 26;
    pub const JLE          : Word = 27;
    // for word
    pub const JA           : Word = 28;
    pub const JAE          : Word = 29;
    pub const JB           : Word = 30;
    pub const JBE          : Word = 31;
    // for real
    pub const FJG          : Word = 32;
    pub const FJGE         : Word = 33;
    pub const FJL          : Word = 34;
    pub const FJLE         : Word = 35;

    // pushes return address and jump by address in dx
    pub const CALL         : Word = 36;
    // pops return address and jump
    pub const RET          : Word = 37;
    // ax - code of syscall
    // bx, cx, dx - args for syscall
    pub const SYSCALL      : Word = 38;

    // moves
    pub const MOVE_OP_TO_AX: Word = 39;
    pub const MOVE_OP_TO_BX: Word = 40;
    pub const MOVE_OP_TO_CX: Word = 41;
    pub const MOVE_OP_TO_DX: Word = 42;
    pub const MOVE_BX_TO_AX: Word = 43;
    pub const MOVE_CX_TO_AX: Word = 44;
    pub const MOVE_DX_TO_AX: Word = 45;
    pub const MOVE_AX_TO_BX: Word = 46;
    pub const MOVE_CX_TO_BX: Word = 47;
    pub const MOVE_DX_TO_BX: Word = 48;
    pub const MOVE_AX_TO_CX: Word = 49;
    pub const MOVE_BX_TO_CX: Word = 50;
    pub const MOVE_DX_TO_CX: Word = 51;
    pub const MOVE_AX_TO_DX: Word = 52;
    pub const MOVE_BX_TO_DX: Word = 53;
    pub const MOVE_CX_TO_DX: Word = 54;
 
    // casts
    pub const CWTOR        : Word = 55;
    pub const CSWTOR       : Word = 56;
    pub const CRTOW        : Word = 57;
    pub const CRTOSW       : Word = 58;

    // ax = *ax
    pub const DEREF        : Word = 59;

}