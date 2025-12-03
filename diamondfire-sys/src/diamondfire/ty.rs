use crate::core::{
    clone::Clone,
    intrinsics::transmute_unchecked,
    macros::derive,
    marker::Copy
};


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_opaque {
    _opaque : *mut u8
}

impl df_opaque {

    #[inline(always)]
    pub unsafe fn assume_string_unchecked(self) -> df_string { unsafe { transmute_unchecked::<df_opaque, df_string>(self) } }

    #[inline(always)]
    pub unsafe fn assume_text_unchecked(self) -> df_text { unsafe { transmute_unchecked::<df_opaque, df_text>(self) } }

    #[inline(always)]
    pub unsafe fn assume_number_unchecked(self) -> df_number { unsafe { transmute_unchecked::<df_opaque, df_number>(self) } }

    #[inline(always)]
    pub unsafe fn assume_location_unchecked(self) -> df_location { unsafe { transmute_unchecked::<df_opaque, df_location>(self) } }

    #[inline(always)]
    pub unsafe fn assume_vector_unchecked(self) -> df_vector { unsafe { transmute_unchecked::<df_opaque, df_vector>(self) } }

    #[inline(always)]
    pub unsafe fn assume_sound_unchecked(self) -> df_sound { unsafe { transmute_unchecked::<df_opaque, df_sound>(self) } }

    #[inline(always)]
    pub unsafe fn assume_particle_unchecked(self) -> df_particle { unsafe { transmute_unchecked::<df_opaque, df_particle>(self) } }

    #[inline(always)]
    pub unsafe fn assume_potion_unchecked(self) -> df_potion { unsafe { transmute_unchecked::<df_opaque, df_potion>(self) } }

    #[inline(always)]
    pub unsafe fn assume_item_unchecked(self) -> df_item { unsafe { transmute_unchecked::<df_opaque, df_item>(self) } }

    #[inline(always)]
    pub unsafe fn assume_list_unchecked(self) -> df_list { unsafe { transmute_unchecked::<df_opaque, df_list>(self) } }

    #[inline(always)]
    pub unsafe fn assume_dict_unchecked(self) -> df_dict { unsafe { transmute_unchecked::<df_opaque, df_dict>(self) } }

}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_string {
    _opaque : df_opaque
}

impl df_string {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
    #[expect(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn from_str(s : &str) -> Self { unsafe { transmute_unchecked::<&str, df_opaque>(s).assume_string_unchecked() } }
    #[inline(always)]
    pub fn into_str(self) -> &'static str { unsafe { transmute_unchecked::<df_opaque, &str>(self._opaque) } }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_text {
    _opaque : df_opaque
}

impl df_text {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_number {
    _opaque : df_opaque
}

impl df_number {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
    #[inline(always)]
    pub fn from_f64(v : f64) -> Self { unsafe { transmute_unchecked::<f64, df_opaque>(v).assume_number_unchecked() } }
    #[inline(always)]
    pub fn into_f64(self) -> f64 { unsafe { transmute_unchecked::<df_opaque, f64>(self._opaque) } }
    #[inline(always)]
    pub fn from_i64(v : i64) -> Self { unsafe { transmute_unchecked::<i64, df_opaque>(v).assume_number_unchecked() } }
    #[inline(always)]
    pub unsafe fn into_i64(self) -> i64 { unsafe { transmute_unchecked::<df_opaque, i64>(self._opaque) } }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_location {
    _opaque : df_opaque
}

impl df_location {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_vector {
    _opaque : df_opaque
}

impl df_vector {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_sound {
    _opaque : df_opaque
}

impl df_sound {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_particle {
    _opaque : df_opaque
}

impl df_particle {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_potion {
    _opaque : df_opaque
}

impl df_potion {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_item {
    _opaque : df_opaque
}

impl df_item {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_list {
    _opaque : df_opaque
}

impl df_list {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_dict {
    _opaque : df_opaque
}

impl df_dict {
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}
