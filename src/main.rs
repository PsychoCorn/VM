use virtual_machine::{image::Image, VirtualMachine};
use crate::virtual_machine::op_codes::OpCode;

mod virtual_machine;

fn main() {
    let i: Image = image!{
        OpCode::MOVE_OP_TO_AX, 0,
        OpCode::MOVE_OP_TO_DX, 0,
        OpCode::MOVE_OP_TO_CX, 6,
        OpCode::SYSCALL,
        OpCode::MOVE_OP_TO_AX, 2,
        OpCode::MOVE_OP_TO_DX, 0,
        OpCode::SYSCALL
    };

    let mut i1 = Image::new();
    i1.emit_str("Hello\n");
    i1.set_entry_point_here();
    i1.emit_from_other(&i);
    println!("{}", i1.get_mnemonics());
    let mut m = VirtualMachine::new();
    m.load_image(&i1).unwrap();
    println!("Exit code: {}", m.execute().unwrap());
}

