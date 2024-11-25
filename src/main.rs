use crate::{machine::Machine, program::get_program};
mod machine;
mod program;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Must have 1 argument");
    }
    let filename = &args[1];
    let source = std::fs::read(filename).expect("Could not read from file!");
    let program = get_program(source);
    let mut machine = Machine::new();
    machine.init(program);
    machine.run();
}
