use core::panic;
use std::{io::Read, process::exit};
use crate::op::Op;
use crate::register::Registers;
use crate::program::Program;
use crate::memory::{Memory, MemoryAddress, Platter};
use crate::instruction::{Instruction};

pub struct Machine {
    mem: Memory, // program "array 0"
    ip: usize,
    r: Registers,
}

impl Machine {
    /* PUBLIC */
    pub fn new() -> Self {
        Self {
            mem: Memory::new(), // program "array 0"
            ip: 0,
            r: Registers::new(),
        }
    }
    pub fn load(&mut self, program: Program) {
        if self.mem.len() < 1 {
            self.mem.alloc(0);
        }
        let zero_addr: MemoryAddress = 0.into();
        self.mem[zero_addr] = program.into();
    }
    pub fn run(&mut self) {
        loop {
            // print!("{}\t| ", self.ip);
            self.act();
            // std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    /* PRIVATE */
    fn peek(&self) -> Instruction {
        let zero_addr: MemoryAddress = 0.into();
        self.mem[zero_addr][self.ip].into()
    }
    fn next(&mut self) -> Instruction {
        let instruction = self.peek();
        self.ip += 1;
        instruction
    }
    fn act(&mut self) {
        let i= self.next();
        // println!("{:#010x}", raw_instruction);
        // println!("{i:?}");
        // println!("{}", i.as_pseudo_assembly());
        // print!("{i.op:?}\t");

        // for ease of type
        let r = &mut self.r;
        let a = i.a;
        let b = i.b;
        let c = i.c;
        let mem = &mut self.mem;

        match i.op {
            Op::Move => {
                if r[c] == 0.into() { return; }
                r[a] = r[b];
            }
            Op::Index => {
                r[a] = mem[r[b]][r[c]].into();
            }
            Op::Amend => {
                mem[r[a]][r[b]] = r[c].into();
            }
            Op::Add => {
                r[a] = r[b] + r[c];
            }
            Op::Mult => {
                r[a] = r[b] * r[c];
            }
            Op::Div => {
                r[a] = r[b] / r[c];
            }
            Op::NotAnd => {
                r[a] = !(r[b] & r[c]);
            }
            Op::Halt => {
                exit(0);
            }
            Op::Alloc => {
                let size: usize = r[c].into();
                self.r[b] = mem.alloc(size).into();
            }
            Op::Aband => {
                let addr: MemoryAddress = r[c].into();
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
                let new_program: Program = mem[r[b]].clone().into();
                self.load(new_program);
                self.ip = self.r[c].into();
            }
            Op::Orth => {
               let tmp: Platter = i.value.into();
               r[i.sa] = tmp.into();
            }
        }
    }
}
