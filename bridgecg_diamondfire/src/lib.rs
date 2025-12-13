use bincode::{ Encode, Decode };

pub mod extern_names;


#[derive(Encode, Decode)]
pub struct BridgeCgFunction {

    /// The name of the function.
    /// This will already have been mangled.
    pub name        : String,

    /// Whether this function can be inlined.
    pub can_inline  : bool,

    /// Whether the signature can be changed.
    /// This is `true` only if the linkage is private.
    pub sig_mutable : bool

}
