use core::panic;
use std::time::Duration;
use std::thread::sleep;
use std::{io::Read, process::exit};
use crate::op::Op;
use crate::register::Registers;
use crate::program::Program;
use crate::memory::{Memory, MemoryAddress, Platter};
use crate::instruction::Instruction;

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
        let i = self.next();
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
            /*
                  The register A receives the value in register B,
                  unless the register C contains 0.
             *
             */
            Op::Move => {
                if r[c] == 0.into() { return; }
                r[a] = r[b];
            }
            /*
                  The register A receives the value stored at offset
                  in register C in the array identified by B.
             *
             */
            Op::Index => {
                r[a] = mem[r[b]][r[c]].into();
            }
            /*
                  The array identified by A is amended at the offset
                  in register B to store the value in register C.
             *
             */
            Op::Amend => {
                mem[r[a]][r[b]] = r[c].into();
            }
            /*
                  The register A receives the value in register B plus 
                  the value in register C, modulo 2^32.
             *
             */
            Op::Add => {
                r[a] = r[b] + r[c];
            }
            /*
                  The register A receives the value in register B times
                  the value in register C, modulo 2^32.
             *
             */
            Op::Mult => {
                r[a] = r[b] * r[c];
            }
            /*
                  The register A receives the value in register B
                  divided by the value in register C, if any, where
                  each quantity is treated as an unsigned 32 bit number.
             *
             */
            Op::Div => {
                r[a] = r[b] / r[c];
            }
            /*
                  Each bit in the register A receives the 1 bit if
                  either register B or register C has a 0 bit in that
                  position.  Otherwise the bit in register A receives
                  the 0 bit.
             *
             */
            Op::NotAnd => {
                r[a] = !(r[b] & r[c]);
            }
            /*
                  The universal machine stops computation.
             *
             */
            Op::Halt => {
                exit(0);
            }
            /*
                  A new array is created with a capacity of platters
                  commensurate to the value in the register C. This
                  new array is initialized entirely with platters
                  holding the value 0. A bit pattern not consisting of
                  exclusively the 0 bit, and that identifies no other
                  active allocated array, is placed in the B register.
             *
             */
            Op::Alloc => {
                let size: usize = r[c].into();
                self.r[b] = mem.alloc(size).into();
            }
            /*
                  The array identified by the register C is abandoned.
                  Future allocations may then reuse that identifier.
             *
             */
            Op::Aband => {
                let addr: MemoryAddress = r[c].into();
                mem.free(addr);
            }
            /*
                  The value in the register C is displayed on the console
                  immediately. Only values between and including 0 and 255
                  are allowed.
             *
             */
            Op::Output => {
                let ch: u32 = r[c].into();
                if ch > 255 {
                    panic!();
                }
                let chars = [ch as u8];
                let print_me = std::str::from_utf8(&chars).unwrap();
                print!("{print_me}");
            }
            /*
                  The universal machine waits for input on the console.
                  When input arrives, the register C is loaded with the
                  input, which must be between and including 0 and 255.
                  If the end of input has been signaled, then the 
                  register C is endowed with a uniform value pattern
                  where every place is pregnant with the 1 bit.
             *
             */
            Op::Input => {
                let mut buf = [0u8; 1];
                r[c] = match std::io::stdin().read(&mut buf) {
                    Err(e) => panic!("Could not read from stdin: {}", e),
                    Ok(0) => 0xffff_ffff,
                    Ok(_) => buf[0] as u32,
                }.into();
            }
            /*
                  The array identified by the B register is duplicated
                  and the duplicate shall replace the '0' array,
                  regardless of size. The execution finger is placed
                  to indicate the platter of this array that is
                  described by the offset given in C, where the value
                  0 denotes the first platter, 1 the second, et
                  cetera.

                  The '0' array shall be the most sublime choice for
                  loading, and shall be handled with the utmost
                  velocity.
             *
             */
            Op::Load => {
                let new_program: Program = mem[r[b]].clone().into();
                self.load(new_program);
                self.ip = self.r[c].into();
            }
            /*
                  The value indicated is loaded into the register A
                  forthwith.
             *
             */
            Op::Orth => {
               let tmp: Platter = i.value.into();
               r[i.sa] = tmp.into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::{Memory, Collection};
    use crate::instruction::RawInstruction;
    use super::*;

    #[test]
    fn test_load() {
        let source: Collection<Platter> = vec![
            0xdeadbeefu32,
            0xbabecafeu32,
        ].into();
        let mut expected = Memory::new();
        expected.alloc(2);
        let zero: MemoryAddress = 0.into();
        expected[zero] = source.clone();

        let p: Program = source.into();
        let mut m = Machine::new();
        m.load(p);
        let got = m.mem;
        assert_eq!(expected, got);
    }

    #[test]
    fn test_cond_move_false() {
        let inst: RawInstruction = Instruction{
            op: Op::Move,
            a: 0.into(),
            b: 1.into(),
            c: 2.into(),
            sa: 0.into(),
            value: 0.into(),
        }.into();
        let code: Platter = inst.into();
        let source: Collection<Platter> = vec![
            code,
        ].into();

        let p: Program = source.into();
        let mut m = Machine::new();
        m.load(p);
        let expected = 0xbabecafe;
        m.r[0.into()] = expected.into();
        m.r[1.into()] = 0xdeadbeef.into();
        m.act();
        let got = m.r[0.into()].into();

        assert_eq!(expected, got);
    }

    #[test]
    fn test_cond_move_true() {
        let inst: RawInstruction = Instruction{
            op: Op::Move,
            a: 0.into(),
            b: 1.into(),
            c: 0.into(),
            sa: 0.into(),
            value: 0.into(),
        }.into();
        let code: Platter = inst.into();
        let source: Collection<Platter> = vec![
            code,
        ].into();

        let p: Program = source.into();
        let mut m = Machine::new();
        m.load(p);
        let expected = 0xdeadbeef;
        m.r[0.into()] = 0xbabecafe.into();
        m.r[1.into()] = expected.into();
        m.act();
        let got = m.r[0.into()].into();

        assert_eq!(expected, got);
    }

    #[test]
    fn test_add() {
        let inst: RawInstruction = Instruction{
            op: Op::Add,
            a: 2.into(),
            b: 1.into(),
            c: 0.into(),
            sa: 0.into(),
            value: 0.into(),
        }.into();
        let code: Platter = inst.into();
        let source: Collection<Platter> = vec![
            code,
        ].into();

        let p: Program = source.into();
        let mut m = Machine::new();
        m.load(p);
        m.r[0.into()] = 3_000_000_000.into();
        m.r[1.into()] = 2_000_000_000.into();
        m.act();
        let expected = 705_032_704u32;
        let got = m.r[2.into()].into();

        assert_eq!(expected, got);
    }

    #[test]
    fn test_mult() {
        let inst: RawInstruction = Instruction{
            op: Op::Mult,
            a: 2.into(),
            b: 1.into(),
            c: 0.into(),
            sa: 0.into(),
            value: 0.into(),
        }.into();
        let code: Platter = inst.into();
        let source: Collection<Platter> = vec![
            code,
        ].into();

        let p: Program = source.into();
        let mut m = Machine::new();
        m.load(p);
        m.r[0.into()] = 900_000.into();
        m.r[1.into()] =   4_773.into();
        m.act();
        let expected = 732_704u32;
        let got = m.r[2.into()].into();

        assert_eq!(expected, got);
    }

    #[test]
    fn test_div() {
        let inst: RawInstruction = Instruction{
            op: Op::Div,
            a: 2.into(),
            b: 0.into(),
            c: 1.into(),
            sa: 0.into(),
            value: 0.into(),
        }.into();
        let code: Platter = inst.into();
        let source: Collection<Platter> = vec![
            code,
        ].into();

        let p: Program = source.into();
        let mut m = Machine::new();
        m.load(p);
        m.r[0.into()] = 900000.into();
        m.r[1.into()] =   4773.into();
        m.act();
        let expected =    188u32;
        let got = m.r[2.into()].into();

        assert_eq!(expected, got);
    }
}

