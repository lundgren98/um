use crate::{
    register,
    macros::*,
    types::u25,
    op::Op,
    memory::Platter,
};

enum RegisterType {
    A(RawInstruction),
    B(RawInstruction),
    C(RawInstruction),
    SA(RawInstruction),
}

pub struct Instruction {
    pub op: Op,
    pub a: register::Index,
    pub b: register::Index,
    pub c: register::Index,
    pub sa: register::Index,
    pub value: u25,
}

#[derive(Clone)]
struct RawInstruction(Platter);
impl_into!(RawInstruction, Platter);
impl_from!(RawInstruction, Platter);

impl Into<u25> for RawInstruction {
    fn into(self) -> u25 {
        self.0.into()
    }
}

const PLATTER_SIZE: u32 = 32;
const OP_SIZE: u32 = 4;
const VALUE_SIZE: u32 = 25;
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
            RegisterType::A(n) => shift(n,A_OFFSET).into(),
            RegisterType::B(n) => shift(n,B_OFFSET).into(),
            RegisterType::C(n) => shift(n,C_OFFSET).into(),
            RegisterType::SA(n) => shift(n,SA_OFFSET).into(),
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

impl From<Platter> for Instruction {
    fn from(value: Platter) -> Self {
        let raw: RawInstruction = value.into();
        raw.into()
    }
}
