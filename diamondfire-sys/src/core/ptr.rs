use crate::core::{
    hint::unreachable_unchecked,
    marker::PointeeSized
};


#[lang = "drop_in_place"]
pub unsafe fn drop_in_place<T : PointeeSized>(_to_drop : *mut T) {
    // Compiler built-in
    unsafe { unreachable_unchecked() }
}
