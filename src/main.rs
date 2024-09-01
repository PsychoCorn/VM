use virtual_machine::image::Image;

mod virtual_machine;

fn main() {
    let mut i = Image::new();
    i.emit_opcode(1);
    i.emit_opcode(2);
    i.set_entry_point_here();
    i.emit_opcode(4);
    i.save_to_file("image.kondra").unwrap();

    let mut i1 = Image::new();
    i1.load_from_file("image.kondra").unwrap();
    i1.emit_opcode(123);
    println!("{i1:?}");
}

