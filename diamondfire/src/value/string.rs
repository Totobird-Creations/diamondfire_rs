use diamondfire_sys::{
    df_string,
    consts::DF_CONST__String,
    action::DF_ACTION__SetVar__String
};
use core::{
    borrow::Borrow,
    fmt::{ self, Display, Formatter },
    ops::Deref, str
};


#[derive(Clone)]
pub struct String {
    raw : &'static df_string
}

impl String {
    #[inline(always)]
    pub fn to_raw(self) -> &'static df_string { self.raw }
    #[inline(always)]
    pub fn from_raw(raw : &'static df_string) -> Self { Self { raw } }
}


impl String {

    #[inline(always)]
    pub fn new() -> String {
        String { raw : DF_CONST__String() }
    }

    // TODO: from_utf8

    // TODO: from_utf8_unchecked

    // TODO: into_bytes

    #[inline(always)]
    pub fn as_str(&self) -> &str { unsafe {
        str::from_raw_parts(self.raw as (*const df_string) as (*const u8), self.len())
    } }

    // TODO: as_mut_str

    #[inline(always)]
    pub fn push_str(&mut self, s : &str) { unsafe {
        DF_ACTION__SetVar__String(
            "No spaces" as (*const str) as (*const df_string),
            self as *mut _,
            self.as_str(),
            s
        )
    } }

    // TODO: extend_from_within

    // TODO: push

    // TODO: as_bytes

    // TODO: truncate

    // TODO: pop

    // TODO: remove

    // TODO: remove_matches

    // TODO: retain

    // TODO: insert

    // TODO: insert_str

    // TODO: as_mut_vec

    // TODO: len

    // TODO: is_empty

    // TODO: split_off

    // TODO: clear

    // TODO: drain

    // TODO: replace_range

    // TODO: into_boxed_str

}


// TODO: Add<&str>


// TODO: AddAssign<&str>


// TODO: AsMut<str>


impl AsRef<str> for String {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}


impl Borrow<str> for String {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}


// TODO: BorrowMut<str>


// TODO: Debug


impl Default for String {
    #[inline(always)]
    fn default() -> Self { Self::new() }
}


impl Deref for String {
    type Target = str;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}


// TODO: DerefMut


impl Display for String {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}


// TODO: Eq


// TODO: Extend<&AsciiChar>


// TODO: Extend<&char>


// TODO: Extend<&str>


// TODO: Extend<AsciiChar>


// TODO: Extend<Box<str>>


// TODO: Extend<Cow<'_, str>>


// TODO: Extend<String>


// TODO: Extend<char>


// TODO: From<&String>


// TODO: From<&mut str>


// TODO: From<&str>


// TODO: From<Box<str>>


// TODO: From<Cow<'_, str>>


// TODO: From<char>


// TODO: FromIterator<&char>


// TODO: FromIterator<&str>


// TODO: FromIterator<Box<str>>


// TODO: FromIterator<Cow<'_, str>>


// TODO: FromIterator<String>


// TODO: FromIterator<char>


// TODO: FromStr


// TODO: Hash


// TODO: Index<I>


// TODO: IndexMut<I>


// TODO: Ord


// TODO: PartialEq<&str>


// TODO: PartialEq<ByteStr>


// TODO: PartialEq<ByteString>


// TODO: PartialEq<Cow<'_, str>>


// TODO: PartialEq<String>


// TODO: PartialEq<str>


// TODO: PartialOrd


// TODO: Pattern


// TODO: StructuralPartialEq


// TODO: TryFrom<&ByteStr>


// TODO: TryFrom<ByteString>


// TODO: TryFrom<Vec<u8>>


impl fmt::Write for String {
    #[inline(always)]
    fn write_str(&mut self, s : &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}
