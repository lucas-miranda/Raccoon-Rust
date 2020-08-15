
pub trait Zero<T> {
    fn zero() -> T;
}

macro_rules! impl_zero {
    ($target_type:ty) => {
        impl Zero<$target_type> for $target_type {
            fn zero() -> $target_type {
                0
            }
        }
    }
}

impl_zero!(usize);
impl_zero!(u8);
impl_zero!(u16);
impl_zero!(u32);
impl_zero!(u64);
impl_zero!(u128);
impl_zero!(isize);
impl_zero!(i8);
impl_zero!(i16);
impl_zero!(i32);
impl_zero!(i64);
impl_zero!(i128);
