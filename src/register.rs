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


pub type IndexType = u3;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Register(      900_000);
        let b = Register(        4_773);
        assert_eq!(a + b, Register(   904773));
    }
    #[test]
    fn test_add_overflow() {
        let c = Register(3_000_000_000);
        let d = Register(2_000_000_000);
        assert_eq!(c + d, Register(705_032_704));
    }
    #[test]
    fn test_mul() {
        let a = Register(900_000);
        let b = Register(  4_773);
        assert_eq!(a * b, Register(732_704));
    }
    #[test]
    fn test_div() {
        let a = Register(900000);
        let b = Register(  4773);
        assert_eq!(a / b, Register(   188));
    }
    #[test]
    fn test_and() {
        let a = Register(0xdeadbeef);
        let b = Register(0xbabecafe);
        assert_eq!(a & b, Register(0x9aac8aee));
    }
    #[test]
    fn test_not() {
        let a = Register(0xffffffff);
        let b = Register(0x00000000);
        assert_eq!(!a, b);
    }
}
