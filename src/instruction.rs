use crate::op::Op;
use crate::register::RegisterIndicator;

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
pub struct Instruction {
    pub op: Op,
    pub a: RegisterIndicator,
    pub b: RegisterIndicator,
    pub c: RegisterIndicator,
    pub sa: RegisterIndicator,
    pub value: u32,
    pub raw: u32,
}

impl Instruction {
    pub fn from_num(n: u32) -> Self {
        let num_op = ((n & OPERATOR) >> OP_OFFSET) as u8;
        let a: RegisterIndicator = ((n & REG_A) >> REG_A_OFFSET).into();
        let b: RegisterIndicator = ((n & REG_B) >> REG_B_OFFSET).into();
        let c: RegisterIndicator = ((n & REG_C) >> REG_C_OFFSET).into();
        let sa: RegisterIndicator = ((n & SPEC_A) >> SPEC_A_OFFSET).into();
        let value = n & VALUE;

        let op = Op::from_num(num_op);
        return Self {
            op,
            a,
            b,
            c,
            sa,
            value,
            raw: n,
        };
    }
    pub fn as_pseudo_assembly(&self) -> String {
        match self.op {
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
