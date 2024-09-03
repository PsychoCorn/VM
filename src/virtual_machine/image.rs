use super::{byte_casts::{from_bytes, GetBytes}, Memory, OpCode, Word};
use std::{fs::{File, OpenOptions}, io::{Read, Write}};
use core::slice::Iter;

#[derive(Debug)]
pub struct Image {
    emit_address     : Word,
    entry_point      : Word,
    image            : Memory,
}

impl Image {
    pub fn new() -> Self {
        Self {
            emit_address : 0,
            entry_point  : 0,
            image        : Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.image.clear();
        self.emit_address = 0;
        self.entry_point = 0;
    }

    pub fn emit_opcode(&mut self, opcode: Word) -> Word {
        self.prepare_space(1);
        let start_address = self.emit_address;

        self.emit(opcode);

        return start_address;
    }

    pub fn emit_opcode_with_operand(
        &mut self, 
        opcode: Word,
        operand: Word,
    ) -> Word {
        self.prepare_space(2);
        let start_address = self.emit_address;

        self.emit(opcode);
        self.emit(operand);

        return start_address;
    }

    #[allow(dead_code)]
    pub fn emit_opcode_with_two_operands(
        &mut self, 
        opcode: Word,
        operand1: Word,
        operand2: Word,
    ) -> Word {
        self.prepare_space(3);
        let start_address = self.emit_address;

        self.emit(opcode);
        self.emit(operand1);
        self.emit(operand2);

        return start_address;
    }

    pub fn emit_str(&mut self, s: &str) -> Word {
        let start_address = self.emit_address;
        let mut str_in_words: Memory = Vec::new();
        for c in s.chars() {
            str_in_words.push(c as Word);
        }

        let words_counter = str_in_words.len();

        self.prepare_space(words_counter);

        self
            .image[self.emit_address..self.emit_address+words_counter]
            .copy_from_slice(&str_in_words);

        self.emit_address += words_counter;

        start_address
    }

    pub fn emit_from_other(&mut self, other: &Self) -> Word {
        let word_counter = other.image.len();
        self.prepare_space(word_counter);

        let start_address = self.emit_address;

        self
            .image[self.emit_address..self.emit_address + word_counter]
            .clone_from_slice(&other.image);

        self.emit_address += word_counter;
        return start_address;
    }

    pub fn write_word(&mut self, address: Word, value: Word) {
        self.prepare_space_in_address(address, 1);
        self.image[address] = value;
    }

    pub fn write_data(&mut self, address: Word, data: &[Word]) {
        let words_count = data.len();
        self.prepare_space_in_address(
            address, 
            words_count
        );

        self 
            .image[address..address+words_count]
            .clone_from_slice(data);
    }

    pub fn read_word(&self, address: Word) -> Word {
        self.image[address]
    }

    pub fn set_entry_point_here(&mut self) {
        self.entry_point = self.emit_address;
    }

    pub fn set_entry_point(
        &mut self, 
        entry_point: Word
    ) -> Result<(), ()> 
    {
        if entry_point >= self.image.len() {
            Err(())
        } else {
            self.entry_point = entry_point;
            Ok(())
        }
    }

    pub fn get_image(&self) -> &[Word] {
        &self.image
    }

    pub fn get_entry_point(&self) -> Word {
        self.entry_point
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), &str> {
        let mut file: File;
        if let Ok(f) = 
            OpenOptions::new().write(true).open(path) 
        {
            file = f;
            file.write(self.entry_point.get_bytes()).unwrap();
            file.write(self.get_image().get_bytes()).unwrap();
            return Ok(());
        }

        if let Ok(f) = File::create(path)  {
            file = f;
            file.write(self.entry_point.get_bytes()).unwrap();
            file.write(self.get_image().get_bytes()).unwrap();
            return Ok(());  
        }
        Err("Opening and creating file were failed")
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), &str> {
        let mut file: File;
        if let Ok(f) = File::open(path) {
            file = f;
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            if let Some(words) = from_bytes::<Word>(&buf) {
                self.entry_point = words[0];
                self.write_data(0, &words[1..]);
                self.emit_address = words.len() - 1;
                Ok(())
            } else {
                Err("Failed to read file")
            }
        } else {
            Err("Opening file was failed")
        }
    }

    fn prepare_space(&mut self, words_count: Word) {
        let required = self.emit_address + words_count;

        if required > self.image.len() {
            self.image.resize(required, 0);
        }
    }

    fn prepare_space_in_address(
        &mut self, address: Word, 
        words_count: Word
    ) {
        let required = address + words_count;

        if required > self.image.len() {
            self.image.resize(required, 0);
        }
    }

    fn emit(&mut self, code: Word) {
        self.image[self.emit_address] = code;
        self.emit_address += 1;
    }

    #[allow(dead_code, unused_variables)]
    pub fn get_mnemonics(&self) -> String {
        self.get_mnemonics_from(self.entry_point)
    }

    #[allow(dead_code, unused_variables)]
    pub fn get_mnemonics_from(&self, address: Word) -> String {
        let mut result = String::new();
        let mut idx = address;

        while idx < self.image.len() {
            result.push_str(format!("0x{idx:0>16x}: ").as_str());
            match self.image[idx] {
                OpCode::ADD => result.push_str("add"),
                OpCode::AND => result.push_str("and"),
                OpCode::CALL => result.push_str("call"),
                OpCode::CRTOSW => result.push_str("crtosw"),
                OpCode::CRTOW => result.push_str("crtow"),
                OpCode::CSWTOR => result.push_str("cswtor"),
                OpCode::CWTOR => result.push_str("cwtor"),
                OpCode::DEC => result.push_str("dec"),
                OpCode::DEREF => result.push_str("deref"),
                OpCode::DIV => result.push_str("div"),
                OpCode::FADD => result.push_str("fadd"),
                OpCode::FDIV => result.push_str("fdiv"),
                OpCode::FJG => result.push_str("fjg"),
                OpCode::FJGE => result.push_str("fjge"),
                OpCode::FJL => result.push_str("fjl"),
                OpCode::FJLE => result.push_str("fjle"),
                OpCode::FMUL => result.push_str("fmul"),
                OpCode::FNEG => result.push_str("fneg"),
                OpCode::FSUB => result.push_str("fsub"),
                OpCode::INC => result.push_str("inc"),
                OpCode::JA => result.push_str("ja"),
                OpCode::JAE => result.push_str("jae"),
                OpCode::JB => result.push_str("jb"),
                OpCode::JBE => result.push_str("jbe"),
                OpCode::JE => result.push_str("je"),
                OpCode::JG => result.push_str("jg"),
                OpCode::JGE => result.push_str("jge"),
                OpCode::JL => result.push_str("jl"),
                OpCode::JLE => result.push_str("jle"),
                OpCode::JMP => result.push_str("jmp"),
                OpCode::JNE => result.push_str("jne"),
                OpCode::MOVE_AX_TO_BX => result.push_str("mov bx, ax"),
                OpCode::MOVE_AX_TO_CX => result.push_str("mov cx, ax"),
                OpCode::MOVE_AX_TO_DX => result.push_str("mov dx, ax"),
                OpCode::MOVE_BX_TO_AX => result.push_str("mov ax, bx"),
                OpCode::MOVE_BX_TO_CX => result.push_str("mov cx, bx"),
                OpCode::MOVE_BX_TO_DX => result.push_str("mov dx, bx"),
                OpCode::MOVE_CX_TO_AX => result.push_str("mov ax, cx"),
                OpCode::MOVE_CX_TO_BX => result.push_str("mov bx, cx"),
                OpCode::MOVE_CX_TO_DX => result.push_str("mov dx, cx"),
                OpCode::MOVE_DX_TO_AX => result.push_str("mov ax, dx"),
                OpCode::MOVE_DX_TO_BX => result.push_str("mov bx, dx"),
                OpCode::MOVE_DX_TO_CX => result.push_str("mov cx, dx"),
                OpCode::MOVE_OP_TO_AX => {
                    result.push_str("mov ax, ");
                    idx += 1;
                    result.push_str(
                        format!("0x{:x}", self.image[idx]).as_str()
                    )
                },
                OpCode::MOVE_OP_TO_BX => {
                    result.push_str("mov bx, ");
                    idx += 1;
                    result.push_str(
                        format!("0x{:x}", self.image[idx]).as_str()
                    )
                },
                OpCode::MOVE_OP_TO_CX => {
                    result.push_str("mov cx, ");
                    idx += 1;
                    result.push_str(
                        format!("0x{:x}", self.image[idx]).as_str()
                    )
                },
                OpCode::MOVE_OP_TO_DX => {
                    result.push_str("mov dx, ");
                    idx += 1;
                    result.push_str(
                        format!("0x{:x}", self.image[idx]).as_str()
                    )
                },
                OpCode::MUL => result.push_str("mul"),
                OpCode::NEG => result.push_str("neg"),
                OpCode::NOT => result.push_str("not"),
                OpCode::OR => result.push_str("or"),
                OpCode::POP => result.push_str("pop"),
                OpCode::PUSH => result.push_str("push"),
                OpCode::RET => result.push_str("ret"),
                OpCode::SHL => result.push_str("shl"),
                OpCode::SHR => result.push_str("shr"),
                OpCode::SUB => result.push_str("sub"),
                OpCode::SYSCALL => result.push_str("syscall"),
                OpCode::XOR => result.push_str("xor"),
                _ => panic!("Unknown opcode")
            }
            result.push('\n');
            idx += 1;
        }

        result
    }

}

#[macro_export]
macro_rules! image {
    ( $( $opcode:expr ),* ) => {
        {
            let mut i = Image::new();

            $(
                i.emit_opcode($opcode as usize);
            )*

            i
        }
    };
}