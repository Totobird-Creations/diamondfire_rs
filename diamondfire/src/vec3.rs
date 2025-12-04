use diamondfire_sys::*;
use diamondfire_macros::string_enum;
use core::{
    intrinsics::transmute_unchecked,
    mem::MaybeUninit,
    ops::{ Add, Sub, Mul, Div }
};


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3 {
    _opaque : df_vector
}


impl Vec3 {

    #[inline(always)]
    pub fn new(x : f64, y : f64, z : f64) -> Self { unsafe {
        let mut out = MaybeUninit::uninit();
        DF_ACTION__setSPECIALSpace_variable__Vector(
            out.as_mut_ptr(),
            df_number::from_f64(x),
            df_number::from_f64(y),
            df_number::from_f64(z)
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Vec3 {

    #[inline(always)]
    pub fn x(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetVectorComp(
            df_string::from_str("X"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    #[inline(always)]
    pub fn y(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetVectorComp(
            df_string::from_str("Y"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    #[inline(always)]
    pub fn z(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetVectorComp(
            df_string::from_str("Z"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

}


impl Vec3 {

    #[inline(always)]
    pub fn with_x(self, x : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetVectorComp(
            df_string::from_str("X"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(x)
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn with_y(self, y : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetVectorComp(
            df_string::from_str("Y"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(y)
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn with_z(self, z : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetVectorComp(
            df_string::from_str("Z"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(z)
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Vec3 {

    #[inline(always)]
    pub fn length(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetVectorLength(
            df_string::from_str("Length"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    #[inline(always)]
    pub fn length_squared(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetVectorLength(
            df_string::from_str("Length Squared"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

}


impl Vec3 {

    #[inline(always)]
    pub fn with_length(self, len : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetVectorLength(
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(len)
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs : Vec3) -> Self::Output { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__AddVectors(
            out.as_mut_ptr(),
            self._opaque,
            rhs._opaque
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs : Vec3) -> Self::Output { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SubtractVectors(
            out.as_mut_ptr(),
            self._opaque,
            rhs._opaque
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs : f64) -> Self::Output { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__MultiplyVector(
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(rhs)
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs : f64) -> Self::Output {
        self * (1.0 / rhs)
    }

}


impl Vec3 {

    #[inline(always)]
    pub fn align(self) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__AddVectors(
            out.as_mut_ptr(),
            self._opaque
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Vec3 {

    #[inline(always)]
    pub fn rotate_x(self, angle : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__RotateAroundAxis(
            df_string::from_str("X"),
            df_string::from_str("Radians"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(angle)
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn rotate_y(self, angle : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__RotateAroundAxis(
            df_string::from_str("Y"),
            df_string::from_str("Radians"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(angle)
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn rotate_z(self, angle : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__RotateAroundAxis(
            df_string::from_str("Z"),
            df_string::from_str("Radians"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(angle)
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn rotate(self, axis : Vec3, angle : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__RotateAroundVec(
            df_string::from_str("Radians"),
            out.as_mut_ptr(),
            self._opaque,
            axis._opaque,
            df_number::from_f64(angle)
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn reflect(self, axis : Vec3) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__ReflectVector(
            out.as_mut_ptr(),
            self._opaque,
            axis._opaque
        );
        Self { _opaque : out.assume_init() }
    } }

}


impl Vec3 {

    #[inline(always)]
    pub fn cross(self, rhs : Vec3) -> Self { unsafe {
        let mut out = MaybeUninit::<df_vector>::uninit();
        DF_ACTION__setSPECIALSpace_variable__CrossProduct(
            out.as_mut_ptr(),
            self._opaque,
            rhs._opaque
        );
        Self { _opaque : out.assume_init() }
    } }

    #[inline(always)]
    pub fn dot(self, rhs : Vec3) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__DotProduct(
            out.as_mut_ptr(),
            self._opaque,
            rhs._opaque
        );
        out.assume_init().into_f64()
    } }

}


#[string_enum]
pub enum Direction {
    #[string_enum_rename = "north"]
    North,
    #[string_enum_rename = "east"]
    East,
    #[string_enum_rename = "south"]
    South,
    #[string_enum_rename = "west"]
    West,
    #[string_enum_rename = "up"]
    Up,
    #[string_enum_rename = "down"]
    Down
}

impl Vec3 {

    #[inline(always)]
    pub fn direction(self) -> Direction { unsafe {
        let mut out = MaybeUninit::<df_string>::uninit();
        DF_ACTION__setSPECIALSpace_variable__DirectionName(
            out.as_mut_ptr(),
            self._opaque
        );
        transmute_unchecked::<df_string, Direction>(out.assume_init())
    } }

}
