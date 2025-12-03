pub const VALID_CHARS : &[char] = &[
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '0','1','2','3','4','5','6','7','8','9'
];

const INVALID_CHARS : &[(char, &str,)] = &[
    (' ', "Space",),
    ('_', "Underscore",),
    ('(', "Leftparen",),
    (')', "Rightparen",),
    ('-', "Hyphen",),
    ('\'', "Apostrophe",),
    ('%', "Percent",),
    ('+', "Plus",),
    ('/', "Slash",),
    ('=', "Equal",),
    ('<', "Lessthan",),
    ('>', "Greaterthan",),
    ('!', "Exclamation",),
    ('[', "Leftbracket",),
    (']', "Rightbracket",)
];

enum CharOrStr<'l> {
    Char(char),
    Str(&'l str)
}
impl<'l> FromIterator<CharOrStr<'l>> for String {
    fn from_iter<T : IntoIterator<Item = CharOrStr<'l>>>(iter : T) -> Self {
        let mut s = String::new();
        for v in iter { match (v) {
            CharOrStr::Char(x) => { s.push(x); },
            CharOrStr::Str(x) => {
                s.push_str("SPECIAL");
                s.push_str(x);
                s.push('_');
            },
        } }
        s
    }
}

pub fn identify(s : &str) -> String {
    s.chars().map(|ch| {
        if (VALID_CHARS.contains(&ch)) {
            CharOrStr::Char(ch)
        } else if let Some(x) = INVALID_CHARS.iter().find_map(|(ch1, replacement,)| (ch == *ch1).then_some(*replacement)) {
            CharOrStr::Str(x)
        } else {
            unimplemented!("identifier character mangling for {ch:?}");
        }
    }).collect::<String>()
}
