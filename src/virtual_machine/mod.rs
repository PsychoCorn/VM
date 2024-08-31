mod lexical_cast;
mod into_char;
pub mod image;
pub mod op_codes;

use std::io::{self, Write};

use image::Image;
use into_char::IntoChar;
use lexical_cast::LexicalCast;
use op_codes::*;

pub type Word  = usize;
pub type SWord = isize;
pub type Real  = f64;

const DEFAULT_MEM_SIZE: Word = 0x10000;

type Memory = Vec<Word>;

/// ip - instraction pointer;
/// sp - stack pointer;
/// fp - frame pointer;
/// lp - local variable pointer
/// ax & bx - registers for operations
#[derive(Debug)]
pub struct VirtualMachine {
    memory     : Memory,

    ip         : Word, 
    sp         : Word,
    fp         : Word,
    lp         : Word,
    ax         : Word,
    bx         : Word,
    cx         : Word,
    dx         : Word,

    max_address: Word,
}

impl VirtualMachine {

    pub fn with_memory(memory_size: Word) -> Self {
        let max_address = memory_size / size_of::<Word>();
        Self {
            max_address,
            ip    : 0,
            sp    : 0,
            fp    : 0,
            lp    : 0,
            ax    : 0,
            bx    : 0,
            cx    : 0,
            dx    : 0,
            memory: vec![0; max_address],
        }
    }

    pub fn new() -> Self {
        Self::with_memory(DEFAULT_MEM_SIZE)
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn max_address(&self) -> Word {
        self.max_address
    }

    pub fn get_ip(&self) -> Word {
        self.ip
    }

    pub fn get_sp(&self) -> Word {
        self.sp
    }

    pub fn get_fp(&self) -> Word {
        self.fp
    }

    pub fn get_lp(&self) -> Word {
        self.lp
    }

    pub fn load_image(&mut self, image: &Image) -> Result<(), &str>
    {
        if image.get_image().len() > self.max_address {
            return Err("image's size too large");
        }

        let i = image.get_image();

        self
            .memory[..i.len()]
            .clone_from_slice(i);

        self.ip = image.get_entry_point();

        Ok(())
    }

    pub fn execute(&mut self) -> Result<Word, ()> {
        self.sp = self.max_address;
        self.fp = self.sp;
        self.lp = self.sp - 1;

        while self.ip < self.max_address {

            match self.memory[self.ip] {

                OpCode::PUSH => {
                    self.sp -= 1;
                    self.memory[self.sp] = self.cx;
                }

                OpCode::POP => {
                    self.cx = self.memory[self.sp];
                    self.sp += 1;
                }

                OpCode::INC => {
                    self.ax = self.ax.wrapping_add(1);
                }

                OpCode::DEC => {
                    self.ax = self.ax.wrapping_sub(1);
                }

                OpCode::NEG => {
                    self.ax = self.ax.wrapping_neg();
                }

                OpCode::ADD => {
                    self.ax = self.ax.wrapping_add(self.bx);
                }

                OpCode::SUB => {
                    self.ax = self.ax.wrapping_sub(self.bx);
                }

                OpCode::MUL => {
                    self.ax = self.ax.wrapping_mul(self.bx);
                }
                
                OpCode::DIV => {
                    self.bx = self.ax.wrapping_rem(self.bx);
                    self.ax = self.ax.wrapping_div(self.bx);
                }

                OpCode::FNEG => {
                    let ax: Real = self.ax.lexical_cast().unwrap();
                    self.ax = (-ax).lexical_cast().unwrap();
                }

                OpCode::FADD => {
                    let ax: Real;
                    let bx: Real;
                    ax = self.ax.lexical_cast().unwrap();
                    bx = self.bx.lexical_cast().unwrap();
                    self.ax = (ax + bx).lexical_cast().unwrap();
                }

                OpCode::FSUB => {
                    let ax: Real;
                    let bx: Real;
                    ax = self.ax.lexical_cast().unwrap();
                    bx = self.bx.lexical_cast().unwrap();
                    self.ax = (ax - bx).lexical_cast().unwrap();
                }

                OpCode::FMUL => {
                    let ax: Real;
                    let bx: Real;
                    ax = self.ax.lexical_cast().unwrap();
                    bx = self.bx.lexical_cast().unwrap();
                    self.ax = (ax * bx).lexical_cast().unwrap();
                }

                OpCode::FDIV => {
                    let ax: Real;
                    let bx: Real;
                    ax = self.ax.lexical_cast().unwrap();
                    bx = self.bx.lexical_cast().unwrap();
                    self.ax = (ax / bx).lexical_cast().unwrap();
                }

                OpCode::AND => {
                    self.ax &= self.bx;
                }

                OpCode::OR => {
                    self.ax |= self.bx;
                }

                OpCode::XOR => {
                    self.ax ^= self.bx;
                }

                OpCode::NOT => {
                    self.ax = !self.ax;
                }

                OpCode::SHL => {
                    self.ax <<= self.bx;
                }

                OpCode::SHR => {
                    self.ax >>= self.bx;
                }

                OpCode::JMP => {
                    self.ip = self.dx;
                    continue;
                }

                OpCode::JE => {
                    if self.ax == self.bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JNE => {
                    if self.ax != self.bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JG => {
                    let ax: SWord = self.ax.lexical_cast().unwrap();
                    let bx: SWord = self.ax.lexical_cast().unwrap();
                    if ax > bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JGE => {
                    let ax: SWord = self.ax.lexical_cast().unwrap();
                    let bx: SWord = self.ax.lexical_cast().unwrap();
                    if ax >= bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JL => {
                    let ax: SWord = self.ax.lexical_cast().unwrap();
                    let bx: SWord = self.ax.lexical_cast().unwrap();
                    if ax < bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JLE => {
                    let ax: SWord = self.ax.lexical_cast().unwrap();
                    let bx: SWord = self.ax.lexical_cast().unwrap();
                    if ax <= bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JA => {
                    if self.ax > self.bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JAE => {
                    if self.ax >= self.bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JB => {
                    if self.ax < self.bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::JBE => {
                    if self.ax <= self.bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::FJG => {
                    let ax: Real = self.ax.lexical_cast().unwrap();
                    let bx: Real = self.ax.lexical_cast().unwrap();
                    if ax > bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::FJGE => {
                    let ax: Real = self.ax.lexical_cast().unwrap();
                    let bx: Real = self.ax.lexical_cast().unwrap();
                    if ax >= bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::FJL => {
                    let ax: Real = self.ax.lexical_cast().unwrap();
                    let bx: Real = self.ax.lexical_cast().unwrap();
                    if ax < bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::FJLE => {
                    let ax: Real = self.ax.lexical_cast().unwrap();
                    let bx: Real = self.ax.lexical_cast().unwrap();
                    if ax <= bx {
                        self.ip = self.dx;
                        continue;
                    }
                }

                OpCode::CALL => {
                    self.sp -= 1;
                    self.memory[self.sp] = self.ip + 1;
                    self.ip = self.dx;
                    continue;
                }

                OpCode::RET => {
                    self.dx = self.memory[self.sp];
                    self.sp += 1;
                    self.ip = self.dx;
                    continue;
                }

                OpCode::SYSCALL => {
                    self.syscall();
                }

                OpCode::MOVE_OP_TO_AX => {
                    self.ip += 1;
                    self.ax = self.memory[self.ip];
                }

                OpCode::MOVE_OP_TO_BX => {
                    self.ip += 1;
                    self.bx = self.memory[self.ip];
                }

                OpCode::MOVE_OP_TO_CX => {
                    self.ip += 1;
                    self.cx = self.memory[self.ip];
                }

                OpCode::MOVE_OP_TO_DX => {
                    self.ip += 1;
                    self.dx = self.memory[self.ip];
                }

                OpCode::MOVE_BX_TO_AX => {
                    self.ax = self.bx;
                }

                OpCode::MOVE_CX_TO_AX => {
                    self.ax = self.cx;
                }

                OpCode::MOVE_DX_TO_AX => {
                    self.ax = self.dx;
                }

                OpCode::MOVE_AX_TO_BX => {
                    self.bx = self.ax;
                }

                OpCode::MOVE_CX_TO_BX => {
                    self.bx = self.cx;
                }

                OpCode::MOVE_DX_TO_BX => {
                    self.bx = self.dx;
                }

                OpCode::MOVE_AX_TO_CX => {
                    self.cx = self.ax;
                }

                OpCode::MOVE_BX_TO_CX => {
                    self.cx = self.bx;
                }

                OpCode::MOVE_DX_TO_CX => {
                    self.cx = self.dx;
                }

                OpCode::MOVE_AX_TO_DX => {
                    self.dx = self.ax;
                }

                OpCode::MOVE_BX_TO_DX => {
                    self.dx = self.bx;
                }

                OpCode::MOVE_CX_TO_DX => {
                    self.dx = self.cx;
                }

                OpCode::CWTOR => {
                    let val = self.ax as Real;
                    self.ax = val.lexical_cast().unwrap();
                }

                OpCode::CSWTOR => {
                    let sw_val: SWord = self.ax.lexical_cast().unwrap();
                    let r_val = sw_val as Real;
                    self.ax = r_val.lexical_cast().unwrap();
                }

                OpCode::CRTOW => {
                    let val: Real = self.ax.lexical_cast().unwrap();
                    self.ax = val as Word;
                }

                OpCode::CRTOSW => {
                    let r_val: Real = self.ax.lexical_cast().unwrap();
                    let sw_val = r_val as SWord;
                    self.ax = sw_val.lexical_cast().unwrap();
                }

                opcode => panic!("Unknown opcode: {opcode}")
            }

            self.ip += 1;
        }

        Ok(self.ax)
    } 

    fn syscall(&mut self) {
        match self.ax {
            // print string in console
            // every char in unicode and stores in Word
            // dx - address of first char
            // cx - length of string
            0 => {
                while self.cx > 0 {
                    print!(
                        "{}", 
                        self
                            .memory[self.dx]
                            .into_char()
                            .unwrap()
                    );
                    self.dx += 1;
                    self.cx -= 1;
                }
                io::stdout().flush().unwrap();
            }

            // get line from console
            // dx - address of buffer
            1 => {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                for c in buf.chars() {
                    self.memory[self.dx] = c as Word;
                    self.dx += 1;
                }
            }

            // end of program
            // dx - return value
            2 => {
                self.ax = self.dx;
                self.ip = self.max_address;
            }

            _ => panic!("Unknown syscall")
        }
    }

}