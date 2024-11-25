use machine::Machine;
use program::Program;
use std::io::Read;
mod instruction;
mod machine;
mod macros;
mod memory;
mod op;
mod program;
mod register;
mod types;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please give 1 argument!");
        return;
    }
    let filename = &args[1];
    let mut source = Vec::<u8>::new();
    std::fs::File::open(filename)
        .expect("Could not open file.")
        .read_to_end(&mut source)
        .expect("Could not read file.");
    let program: Program = source.into();
    let mut machine = Machine::new();
    machine.load(program);
    machine.run();
}
