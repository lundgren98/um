use core::panic;
use std::{io::Read, process::exit};
use crate::op::Op;
use crate::register::Registers;
use crate::program::Program;
use crate::memory::Memory;
use crate::instruction::Instruction;

pub struct Machine {
    arrays: Memory, // program "array 0"
    ip: usize,
    r: Registers,
}

impl Machine {
    /* PUBLIC */
    pub fn new() -> Self {
        Self {
            arrays: Memory::new(), // program "array 0"
            ip: 0,
            r: [0.into(); 8].into(),
        }
    }
    pub fn load(&mut self, program: Program) {
        if self.arrays.len() < 1 {
            self.arrays.alloc(0);
        }
        self.arrays[0] = program.into();
    }
    pub fn run(&mut self) {
        loop {
            print!("{}\t| ", self.ip);
            self.act();
            // sleep(std::time::Duration::from_millis(50));
        }
    }

    /* PRIVATE */
    fn peek(&self) -> u32 {
        self.arrays[0][self.ip]
    }
    fn next(&mut self) -> u32 {
        let instruction = self.peek();
        self.ip += 1;
        instruction
    }
    fn act(&mut self) {
        let raw_instruction = self.next();
        let i = Instruction::from_num(raw_instruction);
        // println!("{:#010x}", raw_instruction);
        // println!("{i:?}");
        println!("{}", i.as_pseudo_assembly());
        let op = i.op;
        // print!("{op:?}\t");

        // for ease of type
        let r = &mut self.r;
        let a = i.a;
        let b = i.b;
        let c = i.c;
        let mem = &mut self.arrays;

        match op {
            Op::Move => {
                if r[c] == 0.into() { return; }
                r[a] = dbg!(r[b]);
            }
            Op::Index => {
                r[a] = dbg!(mem[r[b]][r[c]]).into();
            }
            Op::Amend => {
                mem[r[a]][r[b]] = dbg!(r[c]).into();
            }
            Op::Add => {
                r[a] = dbg!(r[b] + r[c]);
            }
            Op::Mult => {
                r[a] = dbg!(r[b] * r[c]);
            }
            Op::Div => {
                r[a] = dbg!(r[b] / r[c]);
            }
            Op::NotAnd => {
                r[a] = dbg!(!(r[b] & r[c]));
            }
            Op::Halt => {
                exit(0);
            }
            Op::Alloc => {
                let size: usize = r[c].into();
                self.r[b] = mem.alloc(size).into();
            }
            Op::Aband => {
                let addr: usize = dbg!(r[c]).into();
                mem.free(addr);
            }
            Op::Output => {
                let ch: u32 = r[c].into();
                if ch > 255 {
                    panic!();
                }
                let chars = [ch as u8];
                let print_me = std::str::from_utf8(&chars).unwrap();
                print!("{print_me}");
            }
            Op::Input => {
                let mut buf = [0u8; 1];
                r[c] = match std::io::stdin().read(&mut buf) {
                    Err(e) => panic!("Could not read from stdin: {}", e),
                    Ok(0) => 0xffff_ffff,
                    Ok(_) => buf[0] as u32,
                }.into();
            }
            Op::Load => {
                let new_program = mem[dbg!(r[b])].clone();
                mem[0] = new_program;
                self.ip = r[c].into();
            }
            Op::Orth => {
                r[i.sa] = dbg!(i.value).into();
            }
        }
    }
}
