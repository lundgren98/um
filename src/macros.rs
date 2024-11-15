
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
