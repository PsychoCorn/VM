use virtual_machine::{image::Image, op_codes::OpCode, VirtualMachine};

mod virtual_machine;

fn main() {
    let i = create_image();
    let mut machine = VirtualMachine::new();
    machine.load_image(&i).unwrap();
    machine.execute().unwrap();
    println!("\n--------------------------");
    println!("{machine:?}");
}

fn create_image() -> Image {
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
