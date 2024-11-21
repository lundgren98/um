use crate::macros::{
    impl_bitand_assign, impl_from_extend, impl_from_narrow, impl_into, impl_into_extend, set_max,
};
use std::ops::BitAndAssign;

#[expect(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct u3(u8);
impl_into!(u3, u8);
impl_into_extend!(u3, usize);
impl_from_narrow!(u3, u8);
impl_from_extend!(u3, u8, u32);
set_max!(u3, 0b111);
impl_bitand_assign!(u3);

#[expect(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct u4(u8);
impl_into!(u4, u8);
impl_from_narrow!(u4, u8);
set_max!(u4, 0b1111);
impl_bitand_assign!(u4);

#[expect(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct u9(u16);
impl_into!(u9, u16);
impl_from_narrow!(u9, u16);
impl_from_extend!(u9, u16, u32);
set_max!(u9, 0b111111111);
impl_bitand_assign!(u9);

#[expect(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct u25(u32);
impl_into!(u25, u32);
impl_from_narrow!(u25, u32);
set_max!(u25, 0x1ffffff);
impl_bitand_assign!(u25);

impl From<u9> for u3 {
    fn from(x: u9) -> Self {
        let tmp: u16 = x.into();
        let tmp: u8 = tmp as u8;
        Self(tmp).narrow()
    }
}

impl From<u32> for u9 {
    fn from(x: u32) -> Self {
        Self(x as u16).narrow()
    }
}

trait Max {
    fn max() -> Self;
}

trait Narrow {
    fn narrow(self) -> Self;
}

impl<T> Narrow for T
where
    T: Max + BitAndAssign + Clone,
{
    fn narrow(self) -> Self {
        let mut n = self.clone();
        n &= Self::max();
        n
    }
}
