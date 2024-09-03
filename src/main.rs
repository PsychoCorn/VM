use virtual_machine::{image::Image, VirtualMachine};
type op = virtual_machine::op_codes::OpCode;

mod virtual_machine;

fn main() {
    let mut i: Image = image!{
        // sum of squares
        op::POP,
        op::MOVE_CX_TO_DX, // return address
        op::POP, // first op
        op::MOVE_CX_TO_AX,
        op::POP, // second op
        op::MOVE_CX_TO_BX,
        op::MOVE_DX_TO_CX,
        op::PUSH,
        op::MOVE_BX_TO_CX,
        op::MOVE_AX_TO_BX,
        op::MUL,
        op::MOVE_AX_TO_DX,
        op::MOVE_CX_TO_AX,
        op::MOVE_CX_TO_BX,
        op::MUL,
        op::MOVE_DX_TO_BX,
        op::ADD,
        op::RET,
        0,
        0,
        op::MOVE_OP_TO_CX, 4,
        op::PUSH,
        op::MOVE_OP_TO_CX, 8,
        op::PUSH,
        op::MOVE_OP_TO_DX, 0,
        op::CALL,
        op::MOVE_AX_TO_DX,
        op::MOVE_OP_TO_AX, 2,
        op::SYSCALL
    };

    println!("{}", i.get_mnemonics());
    i.set_entry_point(0x14).unwrap();
    i.save_to_file("image.kondra").unwrap();
    let mut vm = VirtualMachine::with_memory(8 * 40);
    vm.load_image(&i).unwrap();
    println!("Return value: {}", vm.execute().unwrap());
}

