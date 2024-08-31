use virtual_machine::image::Image;

mod virtual_machine;

fn main() {
    let mut i = Image::new();
    i.emit_opcode(1);
    i.emit_opcode(2);
    i.set_entry_point_here();
    i.emit_opcode(3);
    i.load_to_file("image.kondra").unwrap();
}

