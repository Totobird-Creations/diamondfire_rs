use crate::ty::*;


unsafe extern "C" {

    /// Creates an empty string value.
    pub safe fn DF_CONST__String() -> *mut df_string;

    /// Creates an empty text value.
    pub safe fn DF_CONST__Text() -> *mut df_text;

    /// Creates a 0 number value.
    ///
    /// This is pretty much pointless, but exists for completeness.
    pub safe fn DF_CONST__Number() -> *mut df_number;

    /// Creates a location with all components set to 0.
    pub safe fn DF_CONST__Location() -> *mut df_location;

    /// Creates a vector with all components set to 0.
    pub safe fn DF_CONST__Vector() -> *mut df_vector;

    /// Creates an arbitrary sound with volume and pitch 0.
    ///
    /// No assumption should be made about the sound type, variant, or key.
    pub safe fn DF_CONST__Sound() -> *mut df_sound;

    /// Creates an arbitrary particle with default properties.
    ///
    /// No assumption should be made about the particle type.
    pub safe fn DF_CONST__Particle() -> *mut df_particle;

    /// Creates an arbitrary potion with infinite duration and amplifier 0 (level 1).
    ///
    /// No assumption should be made about the potion type.
    pub safe fn DF_CONST__Potion() -> *mut df_potion;

    /// Creates an arbitrary item.
    ///
    /// No assumption should be made about the item type or data.
    pub safe fn DF_CONST__Item() -> *mut df_item;

}
