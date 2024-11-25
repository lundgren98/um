
macro_rules! impl_from {
    ($name:ident, $underlying:ty) => {
        impl From<$underlying> for $name {
            fn from(n: $underlying) -> Self {
                Self(n)
            }
        }
    };
}
pub(crate) use impl_from;

macro_rules! impl_from_extend {
    ($name:ident, $underlying:ty, $from:ty) => {
        impl From<$from> for $name {
            fn from(n: $from) -> Self {
                (n as $underlying).into()
            }
        }
    };
}
pub(crate) use impl_from_extend;

macro_rules! impl_from_narrow {
    ($name:ident, $underlying:ty) => {
        impl From<$underlying> for $name {
            fn from(n: $underlying) -> Self {
                Self(n).narrow()
            }
        }
    };
}
pub(crate) use impl_from_narrow;

macro_rules! impl_into {
    ($name:ident, $underlying:ty) => {
        impl Into<$underlying> for $name {
            fn into(self) -> $underlying {
                self.0
            }
        }
    };
}
pub(crate) use impl_into;

#[doc = "Via has Into<into>"]
macro_rules! impl_into_via {
    ($name:ident, $via:ty, $into:ty) => {
        impl Into<$into> for $name {
            fn into(self) -> $into {
                let tmp: $via = self.into();
                tmp.into()
            }
        }
    };
}
pub(crate) use impl_into_via;

macro_rules! impl_into_extend {
    ($name:ident, $into:ty) => {
        impl Into<$into> for $name {
            fn into(self) -> $into {
                self.0.into()
            }
        }
    };
}
pub(crate) use impl_into_extend;

#[doc = "Index must have Into<usize>"]
macro_rules! impl_index {
    ($name:ident, $index:ty, $out:ty) => {
        impl std::ops::Index<$index> for $name {
            type Output = $out;
            fn index(&self, index: $index) -> &Self::Output {
                let i: usize = index.into();
                &self.0[i]
            }
        }
        impl std::ops::IndexMut<$index> for $name {
            fn index_mut(&mut self, index: $index) -> &mut Self::Output {
                let i: usize = index.into();
                &mut self.0[i]
            }
        }
    }
}
pub(crate) use impl_index;

// macro_rules! impl_bitand {
//     ($name:ident) => {
//         impl BitAnd for $name {
//             type Output = Self;
//             fn bitand(self, rhs: Self) -> Self::Output {
//                 Self(self.0 & rhs.0)
//             }
//         }
//     };
// }
// pub(crate) use impl_bitand;

macro_rules! impl_bitand_assign {
    ($name:ident) => {
        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
    };
}
pub(crate) use impl_bitand_assign;

macro_rules! set_max {
    ($name:ident, $max:literal) => {
        impl Max for $name {
            fn max() -> Self {
                Self($max)
            }
        }
    };
}
pub(crate) use set_max;
