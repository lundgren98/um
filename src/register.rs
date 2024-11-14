use std::ops::{Add, BitAnd, Div, Index, IndexMut, Mul, Not};
use std::fmt::Display;

type RegisterType = u32;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Register(RegisterType);

const NUMBER_OF_REGISTERS: usize = 8;
type RegistersType = [Register; NUMBER_OF_REGISTERS];
pub struct Registers(RegistersType);

type RegisterIndicatorType = u8;
#[derive(Debug)]
pub struct RegisterIndicator(RegisterIndicatorType);

/* Register */
impl From<RegisterType> for Register {
    fn from(n: RegisterType) -> Self {
        Register(n)
    }
}
impl Into<RegisterType> for Register {
    fn into(self) -> RegisterType {
        self.0
    }
}
/* Is this necessary ???? */
impl Into<usize> for Register {
    fn into(self) -> usize {
        self.0 as usize
    }
}
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
impl From<RegistersType> for Registers {
    fn from(n: RegistersType) -> Self {
        Registers(n)
    }
}
impl Index<RegisterIndicator> for Registers {
    type Output = Register;
    fn index(&self, index: RegisterIndicator) -> &Self::Output {
        let i: RegisterIndicatorType = index.into();
        &self.0[i as usize]
    }
}
impl IndexMut<RegisterIndicator> for Registers {
    fn index_mut(&mut self, index: RegisterIndicator) -> &mut Self::Output {
        let i: RegisterIndicatorType = index.into();
        &mut self.0[i as usize]
    }
}

/* RegisterIndicator */
impl From<u32> for RegisterIndicator {
    fn from(n: u32) -> Self {
        Self(n as u8)
    }
}
impl Into<RegisterIndicatorType> for RegisterIndicator {
    fn into(self) -> RegisterIndicatorType {
        self.0
    }
}
impl Display for RegisterIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

