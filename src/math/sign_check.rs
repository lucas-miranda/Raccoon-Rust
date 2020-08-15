
pub trait SignCheck {
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
}

macro_rules! impl_sign_check_for_signed {
    ($target_type:ty) => {
        impl SignCheck for $target_type {
            fn is_positive(self) -> bool {
                <$target_type>::is_positive(self)
            }

            fn is_negative(self) -> bool {
                <$target_type>::is_negative(self)
            }
        }
    }
}

macro_rules! impl_sign_check_for_unsigned {
    ($target_type:ty) => {
        impl SignCheck for $target_type {
            fn is_positive(self) -> bool {
                true
            }

            fn is_negative(self) -> bool {
                false
            }
        }
    }
}

impl_sign_check_for_unsigned!(usize);
impl_sign_check_for_unsigned!(u8);
impl_sign_check_for_unsigned!(u16);
impl_sign_check_for_unsigned!(u32);
impl_sign_check_for_unsigned!(u64);
impl_sign_check_for_unsigned!(u128);
impl_sign_check_for_signed!(isize);
impl_sign_check_for_signed!(i8);
impl_sign_check_for_signed!(i16);
impl_sign_check_for_signed!(i32);
impl_sign_check_for_signed!(i64);
impl_sign_check_for_signed!(i128);
