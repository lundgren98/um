use std::io::Read;

#[derive(Debug, PartialEq)]
struct Instruction {
    op: u32,
    a: u32,
    b: u32,
    c: u32,
    sa: u32,
    value: u32,
}

fn parse_instruction(n: u32) -> Instruction {
    Instruction {
        op: (n >> 28) & 0xf,
        a: (n >> 6) & 7,
        b: (n >> 3) & 7,
        c: n & 7,
        sa: (n >> 25) & 7,
        value: n & 0x01ff_ffff,
    }
}

#[derive(Debug, PartialEq)]
pub struct Machine {
    mem: Vec<Option<Vec<u32>>>,
    r: [u32; 8],
    ip: u32,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            mem: vec![None],
            r: [0; 8],
            ip: 0,
        }
    }
    pub fn init(&mut self, program: Vec<u32>) {
        self.mem[0] = program.into();
    }
    pub fn run(&mut self) {
        loop {
            let inst = self.next();
            self.do_instruction(inst);
        }
    }
    fn peek(&self) -> Instruction {
        let raw = match &self.mem[0] {
            None => panic!("Program arr is free! (Did you load the program?)"),
            Some(v) => v[self.ip as usize],
        };
        parse_instruction(raw)
    }
    fn next(&mut self) -> Instruction {
        let inst = self.peek();
        self.ip += 1;
        inst
    }
    fn do_instruction(&mut self, inst: Instruction) {
        let (op, a, b, c, sa, value) =
        (inst.op, inst.a as usize, inst.b as usize, inst.c as usize, inst.sa as usize, inst.value);
        match op {
            0 => self.cond_move(a,b,c),
            1 => self.index(a,b,c),
            2 => self.amend(a,b,c),
            3 => self.add(a,b,c),
            4 => self.mult(a,b,c),
            5 => self.div(a,b,c),
            6 => self.notand(a,b,c),
            7 => self.halt(),
            8 => self.alloc(a,b,c),
            9 => self.abandon(a,b,c),
            10 => self.output(a,b,c),
            11 => self.input(a,b,c),
            12 => self.load(a,b,c),
            13 => self.ortho(sa,value,c),
            _ => panic!("Illegal Operation!"),
        }
    }

    fn cond_move(&mut self, a: usize, b: usize, c: usize) {
        if self.r[c] == 0 {
            return;
        }
        self.r[a] = self.r[b];
    }
    fn index(&mut self, a: usize, b: usize, c: usize) {
        self.r[a] = match &self.mem[self.r[b] as usize] {
            None => panic!("Reading from unallocated addr!"),
            Some(v) => v[self.r[c] as usize],
        };
    }
    fn amend(&mut self, a: usize, b: usize, c: usize) {
        match &mut self.mem[self.r[a] as usize] {
            None => panic!("Writing to unallocated addr!"),
            Some(v) => v[self.r[b] as usize] = self.r[c],
        };
    }
    fn add(&mut self, a: usize, b: usize, c: usize) {
        self.r[a] = self.r[b].wrapping_add(self.r[c]);
    }
    fn mult(&mut self, a: usize, b: usize, c: usize) {
        self.r[a] = self.r[b].wrapping_mul(self.r[c]);
    }
    fn div(&mut self, a: usize, b: usize, c: usize) {
        self.r[a] = self.r[b] / self.r[c];
    }
    fn notand(&mut self, a: usize, b: usize, c: usize) {
        self.r[a] = !(self.r[b] & self.r[c]);
    }
    fn halt(&mut self) {
        std::process::exit(0);
    }
    fn alloc(&mut self, a: usize, b: usize, c: usize) {
        self.r[b] = match self
            .mem
            .iter_mut()
            .enumerate()
            .find(|(i, v)| *i > 0 && v.is_none())
        {
            None => {
                let addr = self.mem.len();
                let new = vec![0; self.r[c] as usize];
                self.mem.push(Some(new));
                addr
            }
            Some((i, v)) => {
                let u = vec![0; self.r[c] as usize];
                *v = Some(u);
                i
            }
        } as u32;
    }
    fn abandon(&mut self, a: usize, b: usize, c: usize) {
        self.mem[self.r[c] as usize] = None;
    }
    fn output(&mut self, a: usize, b: usize, c: usize) {
        if self.r[c] > 255 { panic!("Char gt u8::MAX"); }
        let buf = [self.r[c] as u8];
        let output = String::from_utf8(buf.to_vec())
            .expect("Invalid UTF-8");
        print!("{output}");
    }
    fn input(&mut self, a: usize, b: usize, c: usize) {
        let mut buf = [0u8];
        self.r[c] = match std::io::stdin().read(&mut buf) {
            Err(_) => panic!("Could not read from stdin"),
            Ok(0) => u32::MAX,
            Ok(n) => n as u32,
        };
    }
    fn load(&mut self, a: usize, b: usize, c: usize) {
        self.mem[0] = self.mem[self.r[b] as usize].clone();
        self.ip = self.r[c];
    }
    fn ortho(&mut self, sa: usize, value: u32, c: usize) {
        self.r[sa] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        /*                op  sa                    a   b   c  */
        let raw: u32 = 0b0001_010_0000000000000000_011_100_101;
        /*                       |            value            */
        let expected = Instruction {
            op: 1,
            a: 3,
            b: 4,
            c: 5,
            sa: 2,
            value: 0b11100101,
        };
        let got = parse_instruction(raw);
        assert_eq!(expected, got);
    }
    #[test]
    fn test_move_false() {
        let mut m = Machine::new();
        let (a, b, c) = (1, 2, 3);
        m.r[a] = 5;
        m.r[b] = 7;
        m.r[c] = 0;
        m.cond_move(a, b, c);
        let mut expected = Machine::new();
        expected.r[a] = 5;
        expected.r[b] = 7;
        expected.r[c] = 0;
        assert_eq!(expected, m);
    }
    #[test]
    fn test_move_true() {
        let mut m = Machine::new();
        let (a, b, c) = (1, 2, 3);
        m.r[a] = 5;
        m.r[b] = 7;
        m.r[c] = 9;
        m.cond_move(a, b, c);
        let mut expected = Machine::new();
        expected.r[a] = 7;
        expected.r[b] = 7;
        expected.r[c] = 9;
        assert_eq!(expected, m);
    }
    #[test]
    fn test_index() {
        let mut m = Machine::new();
        m.r[1] = 1337;
        m.r[2] = 7;
        m.r[3] = 9;
        m.r[4] = 8;
        m.r[5] = 0;
        m.alloc(0, 2, 3);
        m.amend(2, 4, 1);
        m.index(5, 2, 4);
        let expected = Machine {
            mem: vec![None, vec![0, 0, 0, 0, 0, 0, 0, 0, 1337].into()],
            r: [0, 1337, 1, 9, 8, 1337, 0, 0],
            ip: 0,
        };
        assert_eq!(expected, m);
    }
    #[test]
    fn test_amend() {}
    #[test]
    fn test_add() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.r[2] = 0xcafe0000;
        m.r[3] = 0x0000babe;
        m.add(1, 2, 3);
        let mut expected = Machine::new();
        expected.r[1] = 0xcafebabe;
        expected.r[2] = 0xcafe0000;
        expected.r[3] = 0x0000babe;
        assert_eq!(expected, m);
    }
    #[test]
    fn test_mult() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.r[2] = 0x00000010;
        m.r[3] = 0x0000babe;
        m.mult(1, 2, 3);
        let mut expected = Machine::new();
        expected.r[1] = 0x000babe0;
        expected.r[2] = 0x00000010;
        expected.r[3] = 0x0000babe;
        assert_eq!(expected, m);
    }
    #[test]
    fn test_div() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.r[2] = 0x000b_abe7;
        m.r[3] = 0x0000_0010;
        m.div(1, 2, 3);
        let mut expected = Machine::new();
        expected.r[1] = 0x0000_babe;
        expected.r[2] = 0x000b_abe7;
        expected.r[3] = 0x0000_0010;
        assert_eq!(expected, m);
    }
    #[test]
    fn test_notand() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.r[2] = 0;
        m.r[3] = 0;
        m.notand(1, 2, 3);
        let mut expected = Machine::new();
        expected.r[1] = u32::MAX;
        expected.r[2] = 0;
        expected.r[3] = 0;
        assert_eq!(expected, m);
    }
    #[test]
    fn test_alloc() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.r[2] = 0;
        m.r[3] = 8;
        m.alloc(1, 2, 3);
        let mut expected = Machine::new();
        expected.r[1] = 0;
        expected.r[2] = 1;
        expected.r[3] = 8;
        expected.mem = vec![None, vec![0; 8].into()];
        assert_eq!(expected, m);
    }
    #[test]
    fn test_abandon() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.r[2] = 0;
        m.r[3] = 8;
        m.alloc(0, 2, 3);
        m.abandon(0, 0, 2);
        let mut expected = Machine::new();
        expected.r[1] = 0;
        expected.r[2] = 1;
        expected.r[3] = 8;
        expected.mem = vec![None, None];
        assert_eq!(expected, m);
    }
    #[test]
    fn test_output() {}
    #[test]
    fn test_input() {}
    #[test]
    fn test_load() {
        let mut m = Machine::new();
        m.r[1] = 1337;
        m.r[2] = 7;
        m.r[3] = 9;
        m.r[4] = 8;
        m.alloc(0, 2, 3);
        m.amend(2, 4, 1);
        m.load(0, 2, 4);
        let expected = Machine {
            mem: vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1337].into(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1337].into(),
            ],
            r: [0, 1337, 1, 9, 8, 0, 0, 0],
            ip: 8,
        };
        assert_eq!(expected, m);
    }
    #[test]
    fn test_ortho() {
        let mut m = Machine::new();
        m.r[1] = 0;
        m.ortho(1, 1337, 0);
        let expected = Machine {
            mem: vec![None],
            r: [0, 1337, 0, 0, 0, 0, 0, 0],
            ip: 0,
        };
        assert_eq!(expected, m);
    }
}
