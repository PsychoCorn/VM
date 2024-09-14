use virtual_machine::{image::Image, VirtualMachine, op_codes::OpCode as OC};
use std::env;


mod virtual_machine;

// all before entry point is data segment
// all procedures below main
// main from entry point to first zero instraction word

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("kondra vm was lunched");
        return;
    }

    match args[1].as_str() {
        "run" => {
            assert_eq!(args.len(), 3);
            let mut vm = VirtualMachine::new();
            let mut i = Image::new();
            if let Err(msg) = 
                i.load_from_file(args[2].as_str()) 
            {
                panic!("{msg}");
            } else if let Err(msg) = vm.load_image(&i) {
                panic!("{msg}");
            } else {
                match vm.execute() {
                    Ok(val) => {
                        println!("Program ended with code: {val}");
                    },
                    Err(_) => {
                        println!("Execution failed");
                    }
                }
            }
        },

        "disasm" => {
            assert_eq!(args.len(), 3);
            let mut i = Image::new();
            if let Err(msg) = 
                i.load_from_file(args[2].as_str()) 
            {
                panic!("{msg}");
            } else {
                println!("{}", i.get_mnemonics());
            }
        }

        "test" => {
            let i = create_hello();
            println!("{i:?}");
            i.save_to_file("image.kondra").unwrap();
        }

        _ => println!("unknown command"),
    }
}

fn create_hello() -> Image {
    let mut i = image! {
        'H''e''l''l''o'' ''W''o''r''l''d''!''\n' 
        0 
        0 
        0 
        0 
    };

    i.set_entry_point_here();

    i.emit_from_other(
        &image! {
            OC::MOVE_OP_TO_DX 0
            OC::MOVE_OP_TO_CX 13
            OC::MOVE_OP_TO_AX 0
            OC::SYSCALL
            OC::MOVE_AX_TO_DX
            OC::MOVE_OP_TO_AX 2
            OC::SYSCALL
        }
    );
    i
}