use core::fmt::{ self, Debug, Formatter };


#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub enum VarScope {
    /// `LINE`
    Local,
    /// `LOCAL`
    ThreadLocal,
    /// `GAME`
    Session,
    /// `SAVE`
    Persistent
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct Local(pub usize);
impl Debug for Local { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    write!(f, "local_{}", self.0)
} }

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct Temporary(pub usize);
impl Temporary {
    pub const ZERO        : Self = Self(0);
    pub const PLACEHOLDER : Self = Self(usize::MAX); // TODO: Remove
}
impl Debug for Temporary { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    write!(f, "temp_{}", self.0)
} }
