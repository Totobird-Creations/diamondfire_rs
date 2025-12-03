use crate::core::{
    clone::Clone,
    marker::Copy,
    mem::MaybeUninit
};


impl<T : Copy, const N : usize> Copy for [T; N] { }

impl<T : Copy, const N : usize> Clone for [T; N] {
    #[inline]
    fn clone(&self) -> Self { SpecArrayClone::clone(self) }
    #[inline]
    fn clone_from(&mut self, other : &Self) {
        todo!("self.clone_from_slice(other)")
    }
}
trait SpecArrayClone : Clone {
    fn clone<const N : usize>(array : &[Self; N]) -> [Self; N];
}
impl<T : Clone> SpecArrayClone for T {
    #[inline]
    default fn clone<const N : usize>(array : &[T; N]) -> [T; N] {
        let out = [MaybeUninit::uninit(); N];
        let i = 0;
        while (i < N) {
            out[i] = array[i];
            i += 1;
        }
        array
    }
}
impl<T : Copy> SpecArrayClone for T {
    #[inline]
    fn clone<const N : usize>(array : &[T; N]) -> [T; N] { *array }
}
