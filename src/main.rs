use virtual_machine::{image::Image, lexical_cast::LexicalCast, op_codes::OpCode, VirtualMachine, Real};

mod virtual_machine;

fn main() {
    let i = create_calc_image();
    let mut machine = VirtualMachine::new();
    machine.load_image(&i).unwrap();
    let r: Real = machine.execute().unwrap().lexical_cast().unwrap();
    println!("\n--------------------------");
    println!("Program execution ended with status {r:.2}");
    println!("--------------------------");
    println!("{machine:?}");
}

fn create_hello_image() -> Image {
    let mut i = Image::new();
    let s = "Hello, World";

    i.emit_str(s);
    i.emit_opcode(0);
    i.set_entry_point_here();
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_AX, 0);
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_DX, 0);
    i.emit_opcode_with_operand(
        OpCode::MOVE_OP_TO_CX, 
        s.len()
    );
    i.emit_opcode(OpCode::SYSCALL);
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_AX, 2);
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_DX, 0);
    i.emit_opcode(OpCode::SYSCALL);

    i
}

fn create_calc_image() -> Image {
    let mut i = Image::new();

    i.set_entry_point_here();
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_AX, 2);
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_BX, 3);
    i.emit_opcode(OpCode::ADD);
    i.emit_opcode(OpCode::CSWTOR);
    i.emit_opcode(OpCode::MOVE_AX_TO_DX);
    i.emit_opcode_with_operand(OpCode::MOVE_OP_TO_AX, 2);
    i.emit_opcode(OpCode::SYSCALL);

    i
}
