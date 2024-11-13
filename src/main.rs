use core::panic;
use std::{io::Read, process::exit, thread::sleep};

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
    /*
     *  op   sa                     a       c
     *  |    |                      |       |
     * vvvv vvv                    v vv    vvv
     * 1100 0001 0000 0000 0000 0000 0000 1101
     *                                 ^^ ^
     *                                  |
     *                                  b
     *
     *
     * op = 12
     * sa = 0
     * a = 0
     * b = 1
     * c = 5
     */
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
    ip: usize,
    r: [u32; 8],
}

impl Machine {
    fn next(&mut self) {
        self.ip += 1;
    }
    fn get_instruction(&self) -> u32 {
        self.arrays[0][self.ip]
    }
    fn act(&mut self) {
        let raw_instruction = self.get_instruction();
        let i = Instruction::from_num(raw_instruction);
        println!("{:x}", raw_instruction);
        println!("{i:?}");
        self.next();
        let op = Op::from_num(i.op);
        print!("{op:?}\t");
        match op {
            Op::Move => {
                if self.r[i.c] == 0 {
                    println!("noop");
                    return;
                }
                println!("r{} = r{}", i.a, i.b);
                self.r[i.a] = self.r[i.b];
            }
            Op::Index => {
                println!("r{} = arr[r{}][r{}]", i.a, i.b, i.c);
                self.r[i.a] = self.arrays[self.r[i.b] as usize][self.r[i.c] as usize];
            }
            Op::Amend => {
                println!("arr[r{}][r{}] = r{}", i.a, i.b, i.c);
                self.arrays[self.r[i.a] as usize][self.r[i.b] as usize] = self.r[i.c];
            }
            Op::Add => {
                println!("r{} = r{} + r{}", i.a, i.b, i.c);
                self.r[i.a] = self.r[i.b].wrapping_add(self.r[i.c]);
            }
            Op::Mult => {
                println!("r{} = r{} * r{}", i.a, i.b, i.c);
                self.r[i.a] = self.r[i.b].wrapping_mul(self.r[i.c]);
            }
            Op::Div => {
                println!("r{} = r{} / r{}", i.a, i.b, i.c);
                self.r[i.a] = self.r[i.b] / self.r[i.c];
            }
            Op::NotAnd => {
                println!("r{} = !(r{} & r{})", i.a, i.b, i.c);
                self.r[i.a] = !(self.r[i.b] & self.r[i.c]);
            }
            Op::Halt => {
                exit(0);
            }
            Op::Alloc => {
                println!("r{} = malloc(r{})", i.b, i.c);
                let size = self.r[i.c] as usize;
                let mem = &mut self.arrays;
                let arr = mem.into_iter().enumerate().find(|(_, x)| x.is_empty());
                let addr = match arr {
                    None => {
                        let mut v = Vec::new();
                        v.resize(size, 0);
                        mem.push(v);
                        mem.len() - 1
                    }
                    Some((index, v)) => {
                        v.resize(size, 0);
                        index
                    }
                };
                self.r[i.b] = addr as u32;
            }
            Op::Aband => {
                println!("free r{}", i.c);
                let mem = &mut self.arrays;
                let addr = self.r[i.c] as usize;
                if addr == 0 {
                    panic!();
                }
                mem[addr].resize(0, 0);
            }
            Op::Output => {
                let c = self.r[i.c];
                if c > 255 {
                    panic!();
                }
                let carr = [c as u8];
                let print_me = std::str::from_utf8(&carr).unwrap();
                print!("{print_me}");
            }
            Op::Input => {
                println!("r{} = input", i.c);
                let mut buf = [0u8; 1];
                self.r[i.c] = match std::io::stdin().read(&mut buf) {
                    Err(_) => 0xffff_ffff,
                    Ok(0) => 0xffff_ffff,
                    Ok(_) => buf[0] as u32,
                };
            }
            Op::Load => {
                println!(
                    "jmp arr[r{}][r{}] ({}:{})",
                    i.b, i.c, self.r[i.b], self.r[i.c]
                );
                let new_program = self.arrays[self.r[i.b] as usize].clone();
                self.arrays[0] = new_program;
                self.ip = self.r[i.c] as usize;
            }
            Op::Orth => {
                println!("r{} = {}", i.sa, i.value);
                self.r[i.sa] = i.value;
            }
        }
    }
    fn run(&mut self) {
        loop {
            print!("{}\t| ", self.ip);
            self.act();
            sleep(std::time::Duration::from_millis(50));
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
    (0..7).for_each(|_| {
        let v = Vec::<u32>::new();
        arrays.push(v);
    });
    let mut machine = Machine {
        arrays, // program "array 0"
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
        let result = align(source);
        assert_eq!(
            result,
            vec![
                0xde000000, 0x00ad0000, 0x0000be00, 0x000000ef, 0xba000000, 0x00be0000, 0x0000ca00,
                0x000000fe,
            ]
        );
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
