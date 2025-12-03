use crate::core::clone::Clone;


#[lang = "copy"]
pub trait Copy : Clone { }

#[rustc_builtin_macro]
pub macro Copy($item:item) {
    /* compiler built-in */
}

impl Copy for u8 { }
impl Copy for i8 { }
impl Copy for u16 { }
impl Copy for i16 { }
impl Copy for u32 { }
impl Copy for i32 { }
impl Copy for u64 { }
impl Copy for i64 { }
impl Copy for u128 { }
impl Copy for i128 { }
impl Copy for usize { }
impl Copy for isize { }
impl Copy for f32 { }
impl Copy for f64 { }

impl<T> Copy for *const T { }
impl<T> Copy for *mut T { }
