use machine::Machine;
use program::Program;
use std::io::Read;
mod instruction;
mod machine;
mod op;
mod register;
mod program;
mod memory;

fn main() {
    let mut source = Vec::<u8>::new();
    std::io::stdin()
        .read_to_end(&mut source)
        .expect("Could not read input");
    let program: Program = source.into();
    let mut machine = Machine::new();
    machine.load(program);
    machine.run();
}
