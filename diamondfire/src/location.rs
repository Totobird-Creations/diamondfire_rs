use crate::Vec3;
use diamondfire_sys::*;
use core::{
    mem::MaybeUninit
};


/// A world location.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Location {
    _opaque : df_location
}


impl Location {

    /// Creates a new south-facing (`+Z`) location at the negative-most corner of the plot.
    #[must_use]
    #[inline(always)]
    pub fn origin() -> Self {
        Self { _opaque : DF_LOCATION__Zero() }
    }

    /// Creates a new location.
    #[must_use]
    #[inline(always)]
    pub fn new(x : f64, y : f64, z : f64, yaw : f64, pitch : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetAllCoords(
            df_string::from_str("Plot coordinate"),
            out.as_mut_ptr(),
            DF_LOCATION__Zero(),
            df_number::from_f64(x),
            df_number::from_f64(y),
            df_number::from_f64(z),
            df_number::from_f64(pitch),
            df_number::from_f64(yaw)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new south-facing (`+Z`) location.
    #[must_use]
    #[inline(always)]
    pub fn from_xyz(x : f64, y : f64, z : f64) -> Self {
        Self::new(x, y, z, 0.0, 0.0)
    }

    /// Creates a new south-facing (`+Z`) location from a vector.
    #[must_use]
    #[inline(always)]
    pub fn from_vec3(v : Vec3) -> Self {
        Self::from_xyz(v.x(), v.y(), v.z())
    }

}


impl Location {

    /// Returns the `x` element of this location.
    #[must_use]
    #[inline(always)]
    pub fn x(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("X"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    /// Returns the `y` element of this location.
    #[must_use]
    #[inline(always)]
    pub fn y(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Y"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    /// Returns the `z` element of this location.
    #[must_use]
    #[inline(always)]
    pub fn z(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Z"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    /// Returns the `yaw` element of this location.
    #[must_use]
    #[inline(always)]
    pub fn yaw(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Yaw"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    /// Returns the `pitch` element of this location.
    #[must_use]
    #[inline(always)]
    pub fn pitch(self) -> f64 { unsafe {
        let mut out = MaybeUninit::<df_number>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Pitch"),
            out.as_mut_ptr(),
            self._opaque
        );
        out.assume_init().into_f64()
    } }

    /// Returns the `x`, `y`, and `z` elements of this location as a [`Vec3`].
    #[must_use]
    #[inline(always)]
    pub fn xyz(self) -> Vec3 {
        Vec3::new(self.x(), self.y(), self.z())
    }

}


impl Location {

    /// Creates a new location by replacing the `x` element of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_x(self, x : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("X"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(x)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new location by replacing the `y` element of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_y(self, y : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Y"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(y)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new location by replacing the `z` element of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_z(self, z : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Z"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(z)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new location by replacing the `yaw` element of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_yaw(self, yaw : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Yaw"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(yaw)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new location by replacing the `x` element of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_pitch(self, pitch : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetCoord(
            df_string::from_str("Plot coordinate"),
            df_string::from_str("Pitch"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(pitch)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new location by replacing the `x`, `y`, and `z` elements of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_xyz(self, x : f64, y : f64, z : f64) -> Self { unsafe {
        let mut out = MaybeUninit::<df_location>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetAllCoords(
            df_string::from_str("Plot coordinate"),
            out.as_mut_ptr(),
            self._opaque,
            df_number::from_f64(x),
            df_number::from_f64(y),
            df_number::from_f64(z)
        );
        Self { _opaque : out.assume_init() }
    } }

    /// Creates a new location by replacing the `yaw` and `pitch` elements of `self`.
    #[must_use]
    #[inline(always)]
    pub fn with_rot(self, yaw : f64, pitch : f64) -> Self {
        Self::new(self.x(), self.y(), self.z(), yaw, pitch)
    }

}
