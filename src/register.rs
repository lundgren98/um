use crate::{
    macros::{impl_from, impl_into, impl_into_via, impl_into_extend, impl_index},
    memory::{MemoryAddress, Platter},
    types::u3,
};
use std::ops::{Add, BitAnd, Div, Mul, Not};

type RegisterType = Platter;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Register(RegisterType);
impl_from!(Register, RegisterType);
impl_into!(Register, RegisterType);
impl Into<usize> for Register {
    fn into(self) -> usize {
        self.0 as usize
    }
}


type IndexType = u3;
#[derive(Debug)]
pub struct Index(IndexType);
impl_from!(Index, IndexType);
impl_into!(Index, IndexType);
impl_into_extend!(Index, usize);

const NUMBER_OF_REGISTERS: usize = 8;
type RegistersType = [Register; NUMBER_OF_REGISTERS];
pub struct Registers(RegistersType);
impl_from!(Registers, RegistersType);
impl_index!(Registers, Index, Register);

/* Register */
impl Add for Register {
    type Output = Register;
    fn add(self, other: Self) -> Self::Output {
        (self.0.wrapping_add(other.0)).into()
    }
}
impl Mul for Register {
    type Output = Register;
    fn mul(self, other: Self) -> Self::Output {
        (self.0.wrapping_mul(other.0)).into()
    }
}
impl Div for Register {
    type Output = Register;
    fn div(self, other: Self) -> Self::Output {
        (self.0 / other.0).into()
    }
}
impl BitAnd for Register {
    type Output = Register;
    fn bitand(self, other: Self) -> Self::Output {
        (self.0 & other.0).into()
    }
}
impl Not for Register {
    type Output = Register;
    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}


/* Registers */
impl Registers {
    pub fn new() -> Self {
        let zero: Register = 0.into();
        Registers([zero; NUMBER_OF_REGISTERS])
    }
}

/* Index */
impl From<u32> for Index {
    fn from(value: u32) -> Self {
        let underlying: IndexType = value.into();
        underlying.into()
    }
}
