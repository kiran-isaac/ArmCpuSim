mod program;

fn main() {
    let elf = program::Program::load("thumb/test.o");
    println!("{:02X?}", elf.get_text());
}
