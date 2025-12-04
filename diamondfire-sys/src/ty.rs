use core::intrinsics::transmute_unchecked;


/// An untyped DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_opaque {
    _opaque : u8
}

impl df_opaque {

    /// Assumes that this value is a DiamondFire `String` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `String`.
    #[inline(always)]
    pub unsafe fn assume_string_unchecked(self) -> df_string { unsafe { transmute_unchecked::<df_opaque, df_string>(self) } }

    /// Assumes that this value is a DiamondFire `Styled Text` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Styled Text`.
    #[inline(always)]
    pub unsafe fn assume_text_unchecked(self) -> df_text { unsafe { transmute_unchecked::<df_opaque, df_text>(self) } }

    /// Assumes that this value is a DiamondFire `Number` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Number`.
    #[inline(always)]
    pub unsafe fn assume_number_unchecked(self) -> df_number { unsafe { transmute_unchecked::<df_opaque, df_number>(self) } }

    /// Assumes that this value is a DiamondFire `Location` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Location`.
    #[inline(always)]
    pub unsafe fn assume_location_unchecked(self) -> df_location { unsafe { transmute_unchecked::<df_opaque, df_location>(self) } }

    /// Assumes that this value is a DiamondFire `Vector` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Vector`.
    #[inline(always)]
    pub unsafe fn assume_vector_unchecked(self) -> df_vector { unsafe { transmute_unchecked::<df_opaque, df_vector>(self) } }

    /// Assumes that this value is a DiamondFire `Sound` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Sound`.
    #[inline(always)]
    pub unsafe fn assume_sound_unchecked(self) -> df_sound { unsafe { transmute_unchecked::<df_opaque, df_sound>(self) } }

    /// Assumes that this value is a DiamondFire `Particle` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Particle`.
    #[inline(always)]
    pub unsafe fn assume_particle_unchecked(self) -> df_particle { unsafe { transmute_unchecked::<df_opaque, df_particle>(self) } }

    /// Assumes that this value is a DiamondFire `Potion` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Potion`.
    #[inline(always)]
    pub unsafe fn assume_potion_unchecked(self) -> df_potion { unsafe { transmute_unchecked::<df_opaque, df_potion>(self) } }

    /// Assumes that this value is a DiamondFire `Item` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Item`.
    #[inline(always)]
    pub unsafe fn assume_item_unchecked(self) -> df_item { unsafe { transmute_unchecked::<df_opaque, df_item>(self) } }

    /// Assumes that this value is a DiamondFire `List` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `List`.
    #[inline(always)]
    pub unsafe fn assume_list_unchecked(self) -> df_list { unsafe { transmute_unchecked::<df_opaque, df_list>(self) } }

    /// Assumes that this value is a DiamondFire `Dictionary` without checking.
    ///
    /// ## Safety
    /// The underlying value must be a DiamondFire `Dictionary`.
    #[inline(always)]
    pub unsafe fn assume_dict_unchecked(self) -> df_dict { unsafe { transmute_unchecked::<df_opaque, df_dict>(self) } }

}


/// A `String` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_string {
    _opaque : df_opaque
}

impl df_string {

    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }

    /// Converts an `&str` to a DiamondFire `String`.
    #[expect(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn from_str(s : &str) -> Self { unsafe { transmute_unchecked::<&str, df_opaque>(s).assume_string_unchecked() } }

    /// Returns `self` as an `&'static str`.
    #[inline(always)]
    pub fn into_str(self) -> &'static str { unsafe { transmute_unchecked::<df_opaque, &str>(self._opaque) } }

}


/// A `Styled Text` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_text {
    _opaque : df_opaque
}

impl df_text {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `Number` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_number {
    _opaque : df_opaque
}

impl df_number {

    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }

    /// Converts an `f64` to a DiamondFire `String`.
    #[inline(always)]
    pub fn from_f64(v : f64) -> Self { unsafe { transmute_unchecked::<f64, df_opaque>(v).assume_number_unchecked() } }

    /// Returns `self` as an `f64`.
    #[inline(always)]
    pub fn into_f64(self) -> f64 { unsafe { transmute_unchecked::<df_opaque, f64>(self._opaque) } }

    /// Converts an `i64` to a DiamondFire `String`.
    #[inline(always)]
    pub fn from_i64(v : i64) -> Self { unsafe { transmute_unchecked::<i64, df_opaque>(v).assume_number_unchecked() } }

    /// Returns `self` as an `i64`.
    ///
    /// ## Safety
    /// The underlying value must be an integer (`self.fract() == 0.0`).
    #[inline(always)]
    pub unsafe fn into_i64(self) -> i64 { unsafe { transmute_unchecked::<df_opaque, i64>(self._opaque) } }

}


/// A `Location` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_location {
    _opaque : df_opaque
}

impl df_location {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `Vector` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_vector {
    _opaque : df_opaque
}

impl df_vector {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `Sound` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_sound {
    _opaque : df_opaque
}

impl df_sound {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `Particle` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_particle {
    _opaque : df_opaque
}

impl df_particle {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `Potion` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_potion {
    _opaque : df_opaque
}

impl df_potion {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// An `Item` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_item {
    _opaque : df_opaque
}

impl df_item {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `List` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_list {
    _opaque : df_opaque
}

impl df_list {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}


/// A `Dictionary` DiamondFire value.
///
/// This type is 'opaque'. No assumptions should be made about the it or any underlying data.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct df_dict {
    _opaque : df_opaque
}

impl df_dict {
    /// Returns `self` as an untyped DiamondFire value.
    ///
    /// This does not change the underlying value in any way.
    #[inline(always)]
    pub fn into_opaque(self) -> df_opaque { self._opaque }
}
