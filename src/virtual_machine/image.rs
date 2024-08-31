use super::{Memory, Word};

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
            .image[self.emit_address..]
            .copy_from_slice(&str_in_words);

        self.emit_address += words_counter;

        start_address
    }

    pub fn emit_from_other(&mut self, other: &Self) -> Word {
        let word_counter = other.image.len();
        self.prepare_space(word_counter);

        let start_address = self.emit_address;

        self
            .image[self.emit_address..]
            .clone_from_slice(&other.image);

        self.emit_address += word_counter;
        return start_address;
    }

    pub fn write_word(&mut self, address: Word, value: Word) {
        self.prepare_space_in_address(address, 1);
        self.image[address] = value;
    }

    pub fn write_data(&mut self, address: Word, data: &[Word]) {
        self.prepare_space_in_address(
            address, 
            data.len()
        );

        self 
            .image[address..]
            .clone_from_slice(data);
    }

    pub fn read_word(&self, address: Word) -> Word {
        self.image[address]
    }

    pub fn set_entry_point_here(&mut self) {
        self.entry_point = self.emit_address;
    }

    pub fn get_image(&self) -> &[Word] {
        &self.image
    }

    pub fn get_entry_point(&self) -> Word {
        self.entry_point
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
    fn print_mnemonics(&self, address: Word) -> Word {
        todo!()
    }
}