use core::{panic, time};
use std::{collections::HashSet, io::Read, process::exit, thread::sleep};

/* sizes in bits */
const PLATTER_SIZE: u32 = 32;
const OP_SIZE: u32 = 4;
const REG_SIZE: u32 = 3;

/* offsets */
const OP_OFFSET: u32 = PLATTER_SIZE - OP_SIZE;
const OPERATOR: u32 = 0xf << OP_OFFSET;
const SPEC_A_OFFSET: u32 = OP_OFFSET - REG_SIZE;
const REG_C_OFFSET: u32 = 0;
const REG_B_OFFSET: u32 = REG_C_OFFSET + REG_SIZE;
const REG_A_OFFSET: u32 = REG_B_OFFSET + REG_SIZE;

/* bitmasks */
const SPEC_A: u32 = 7 << SPEC_A_OFFSET;
const REG_A: u32 = 7 << REG_A_OFFSET;
const REG_B: u32 = 7 << REG_B_OFFSET;
const REG_C: u32 = 7 << REG_C_OFFSET;
const VALUE: u32 = !(OPERATOR | SPEC_A);

#[derive(Debug)]
struct Instruction {
    op: u8,
    a: usize,
    b: usize,
    c: usize,
    sa: usize,
    value: u32,
}

impl Instruction {
    fn from_num(n: u32) -> Self {
        let op = ((n & OPERATOR) >> OP_OFFSET) as u8;
        let a = ((n & REG_A) >> REG_A_OFFSET) as usize;
        let b = ((n & REG_B) >> REG_B_OFFSET) as usize;
        let c = ((n & REG_C) >> REG_C_OFFSET) as usize;
        let sa = ((n & SPEC_A) >> SPEC_A_OFFSET) as usize;
        let value = n & VALUE;
        return Self {
            op,
            a,
            b,
            c,
            sa,
            value,
        };
    }
    fn as_psuedo_assembly(&self) -> String {
        match Op::from_num(self.op) {
            Op::Move => {
                format!("r{} = r{} if r{}", self.a, self.b, self.c)
            }
            Op::Index => {
                format!("r{} = arr[r{}][r{}]", self.a, self.b, self.c)
            }
            Op::Amend => {
                format!("arr[r{}][r{}] = r{}", self.a, self.b, self.c)
            }
            Op::Add => {
                format!("r{} = r{} + r{}", self.a, self.b, self.c)
            }
            Op::Mult => {
                format!("r{} = r{} * r{}", self.a, self.b, self.c)
            }
            Op::Div => {
                format!("r{} = r{} / r{}", self.a, self.b, self.c)
            }
            Op::NotAnd => {
                format!("r{} = !(r{} & r{})", self.a, self.b, self.c)
            }
            Op::Halt => "HALT".to_string(),
            Op::Alloc => {
                format!("r{} = ALLOC r{}", self.b, self.c)
            }
            Op::Aband => {
                format!("FREE r{}", self.c)
            }
            Op::Output => {
                format!("print r{}", self.c)
            }
            Op::Input => {
                format!("r{} = self.put", self.c)
            }
            Op::Load => {
                format!("jmp arr[r{}][r{}]", self.b, self.c)
            }
            Op::Orth => {
                format!("r{} = {}", self.sa, self.value)
            }
        }
    }
}

#[derive(Debug)]
enum Op {
    Move,
    Index,
    Amend,
    Add,
    Mult,
    Div,
    NotAnd,
    Halt,
    Alloc,
    Aband,
    Output,
    Input,
    Load,
    Orth,
}

impl Op {
    fn from_num(op: u8) -> Self {
        match op {
            0 => Self::Move,
            1 => Self::Index,
            2 => Self::Amend,
            3 => Self::Add,
            4 => Self::Mult,
            5 => Self::Div,
            6 => Self::NotAnd,
            7 => Self::Halt,
            8 => Self::Alloc,
            9 => Self::Aband,
            10 => Self::Output,
            11 => Self::Input,
            12 => Self::Load,
            13 => Self::Orth,
            _ => panic!(),
        }
    }
}

struct Machine {
    arrays: Vec<Vec<u32>>, // program "array 0"
    allocated: Vec<usize>,
    ip: usize,
    r: [u32; 8],
}

impl Machine {
    fn peek(&self) -> u32 {
        self.arrays[0][self.ip]
    }
    fn next(&mut self) -> u32 {
        let instruction = self.peek();
        self.ip += 1;
        instruction
    }
    fn is_free(&self, addr: usize) -> bool {
        self.allocated.iter().find(|x| **x == addr).is_none()
    }
    fn alloc(&mut self, size: usize) -> u32 {
        let all_addresses = HashSet::<usize>::from_iter(1..self.arrays.len());
        let free_addresses = &all_addresses - &(self.allocated.iter().cloned().collect());
        let free_addr = free_addresses.iter().find(|_| true); // Just give me an element
        let addr = match free_addr {
            None => {
                let len = self.arrays.len();
                if len > u32::MAX as usize {
                    panic!("Trying to allocate more memory than the machine is allowed to have.");
                }
                let v = vec![0u32; size];
                self.arrays.push(v);
                self.allocated.push(len);
                len
            }
            Some(&i) => {
                self.arrays[i].resize(size, 0);
                i
            }
        };
        dbg!(addr) as u32
    }
    fn free(&mut self, addr: usize) {
        if addr == 0 {
            panic!("Cannot abandon array 0");
        }
        match self.allocated.iter().enumerate().find(|(_, &a)| a == addr) {
            None => panic!("Cannot abandon unallocated array {}", addr),
            Some((i, _)) => {
                self.allocated.remove(i);
            }
        }
        let size = self.arrays[addr].len();
        self.arrays[addr].resize(0, 0);
        assert_eq!(self.arrays[addr].len(), 0);
        if size == 0 {
            println!("-------\nZERO SIZE FREE OK\n-------");
            sleep(time::Duration::from_secs(1));
        }
    }
    fn act(&mut self) {
        let raw_instruction = self.next();
        let i = Instruction::from_num(raw_instruction);
        // println!("{:#010x}", raw_instruction);
        // println!("{i:?}");
        let op = Op::from_num(i.op);
        // print!("{op:?}\t");
        println!("{}", i.as_psuedo_assembly());

        // for ease of type
        let r = &mut self.r;
        let a = i.a;
        let b = i.b;
        let c = i.c;
        let mem = &mut self.arrays;

        match op {
            Op::Move => {
                if r[c] == 0 { return; }
                r[a] = dbg!(r[b]);
            }
            Op::Index => {
                r[a] = dbg!(mem[r[b] as usize][r[c] as usize]);
            }
            Op::Amend => {
                mem[r[a] as usize][r[b] as usize] = dbg!(r[c]);
            }
            Op::Add => {
                r[a] = dbg!(r[b].wrapping_add(r[c]));
            }
            Op::Mult => {
                r[a] = dbg!(r[b].wrapping_mul(r[c]));
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
                let size = r[c] as usize;
                self.r[b] = self.alloc(size);
            }
            Op::Aband => {
                let addr = dbg!(r[c]) as usize;
                self.free(addr);
            }
            Op::Output => {
                println!("-------\nOUTPUTTING\n-------");
                sleep(time::Duration::from_secs(1));
                let ch = r[c];
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
                };
            }
            Op::Load => {
                let new_program = mem[dbg!(r[b]) as usize].clone();
                mem[0] = new_program;
                self.ip = r[c] as usize;
            }
            Op::Orth => {
                r[i.sa] = dbg!(i.value);
            }
        }
    }
    fn run(&mut self) {
        loop {
            print!("{}\t| ", self.ip);
            self.act();
            // sleep(std::time::Duration::from_millis(50));
        }
    }
}

fn align(input: Vec<u8>) -> Vec<u32> {
    let aligned: Vec<u32> = input
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let offset = ((3 - (i % 4)) % 4) * 8;
            assert!(offset < OP_OFFSET as usize);
            let aligned = (c as u32) << offset;
            aligned
        })
        .collect();
    aligned
}

fn fuse(aligned: Vec<u32>) -> Vec<u32> {
    let program: Vec<u32> = (0..aligned.len() / 4)
        .map(|i| {
            let fourth = i * 4;
            aligned[fourth..fourth + 4].into_iter().sum()
        })
        .collect();
    program
}

fn get_program(input: Vec<u8>) -> Vec<u32> {
    let aligned = align(input);
    let program = fuse(aligned);
    program
}

fn main() {
    let mut source = Vec::<u8>::new();
    std::io::stdin()
        .read_to_end(&mut source)
        .expect("Could not read input");
    let program = get_program(source);
    let mut arrays = Vec::<Vec<u32>>::new();
    arrays.push(program);
    let mut machine = Machine {
        arrays, // program "array 0"
        allocated: vec![0usize],
        ip: 0,
        r: [0; 8],
    };
    machine.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align() {
        let source: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0xba, 0xbe, 0xca, 0xfe];
        let got = align(source);
        let expected = vec![
            0xde000000, 0x00ad0000, 0x0000be00, 0x000000ef, 0xba000000, 0x00be0000, 0x0000ca00,
            0x000000fe,
        ];
        assert_eq!(got, expected);
    }

    #[test]
    fn test_fuse() {
        let nums: Vec<u32> = vec![
            0xde000000, 0x00ad0000, 0x0000be00, 0x000000ef, 0xba000000, 0x00be0000, 0x0000ca00,
            0x000000fe,
        ];
        let got = fuse(nums);
        let expected: Vec<u32> = vec![0xdeadbeef, 0xbabecafe];
        assert_eq!(got, expected);
    }

    #[test]
    fn parse_program() {
        let source: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0xba, 0xbe, 0xca, 0xfe];
        let got = get_program(source);
        let expected: Vec<u32> = vec![0xdeadbeef, 0xbabecafe];
        assert_eq!(got, expected);
    }

    #[test]
    fn test_value() {
        assert_eq!(VALUE, 0x1FFFFFF)
    }

    #[test]
    fn test_operator() {
        assert_eq!(OPERATOR, 0xF000_0000)
    }
}
