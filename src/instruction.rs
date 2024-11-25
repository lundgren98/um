use crate::{macros::*, memory::Platter, op::Op, register, types::u25};

enum RegisterType {
    A(RawInstruction),
    B(RawInstruction),
    C(RawInstruction),
    SA(RawInstruction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub op: Op,
    pub a: register::Index,
    pub b: register::Index,
    pub c: register::Index,
    pub sa: register::Index,
    pub value: u25,
}

#[derive(Clone)]
pub struct RawInstruction(Platter);
impl_into!(RawInstruction, Platter);
impl_from!(RawInstruction, Platter);

impl Into<u25> for RawInstruction {
    fn into(self) -> u25 {
        self.0.into()
    }
}

const PLATTER_SIZE: u32 = 32;
const OP_SIZE: u32 = 4;
// const VALUE_SIZE: u32 = 25;
const OP_OFFSET: u32 = PLATTER_SIZE - OP_SIZE;
impl Into<Op> for RawInstruction {
    fn into(self) -> Op {
        ((self.0 >> OP_OFFSET) as u8).into()
    }
}

const REG_SIZE: u32 = 3;
const A_OFFSET: u32 = REG_SIZE * 2;
const B_OFFSET: u32 = REG_SIZE * 1;
const C_OFFSET: u32 = REG_SIZE * 0;
const SA_OFFSET: u32 = OP_OFFSET - REG_SIZE;
impl From<RegisterType> for register::Index {
    fn from(value: RegisterType) -> Self {
        let shift = |n: RawInstruction, s: u32| {
            let x: u32 = n.into();
            x >> s
        };
        match value {
            RegisterType::A(n) => shift(n, A_OFFSET).into(),
            RegisterType::B(n) => shift(n, B_OFFSET).into(),
            RegisterType::C(n) => shift(n, C_OFFSET).into(),
            RegisterType::SA(n) => shift(n, SA_OFFSET).into(),
        }
    }
}

impl From<RawInstruction> for Instruction {
    fn from(raw: RawInstruction) -> Self {
        let raw = || raw.clone();
        Self {
            op: raw().into(),
            a: RegisterType::A(raw()).into(),
            b: RegisterType::B(raw()).into(),
            c: RegisterType::C(raw()).into(),
            sa: RegisterType::SA(raw()).into(),
            value: raw().into(),
        }
    }
}

impl Into<RawInstruction> for Instruction {
    fn into(self) -> RawInstruction {
        let shift = |&n: &u32, s: u32| n << s;
        let sa: u32 = Into::<register::IndexType>::into(self.sa).into();
        let a: u32 = Into::<register::IndexType>::into(self.a).into();
        let b: u32 = Into::<register::IndexType>::into(self.b).into();
        let c: u32 = Into::<register::IndexType>::into(self.c).into();
        let isa = shift(&sa, SA_OFFSET);
        let ia = shift(&a, A_OFFSET);
        let ib = shift(&b, B_OFFSET);
        let ic = shift(&c, C_OFFSET);
        let ivalue: u32 = self.value.into();
        let ret: u32 = match self.op {
            Op::Orth => {
                let iop = shift(&self.op.into(), OP_OFFSET);
                iop + isa + ivalue
            }
            _ => {
                let iop = shift(&self.op.into(), OP_OFFSET);
                iop + ia + ib + ic
            }
        };
        ret.into()
    }
}

impl From<Platter> for Instruction {
    fn from(value: Platter) -> Self {
        let raw: RawInstruction = value.into();
        raw.into()
    }
}
