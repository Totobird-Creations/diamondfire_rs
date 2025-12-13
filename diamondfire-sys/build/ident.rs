use core::{
    char::ToLowercase,
    fmt::Write as _
};


pub const VALID_CHARS : &[char] = &[
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '0','1','2','3','4','5','6','7','8','9'
];

const INVALID_CHARS : &[(char, &str,)] = &[
    (' ', "",),
    ('(', "",),
    (')', "",),
    ('[', "",),
    (']', "",),
    ('+', "Add",),
    ('-', "Sub",),
    ('/', "Div",),
    ('%', "Mod",),
    ('=', "Equal",),
    ('<', "Lt",),
    ('>', "Gt",),
    ('!', "Not",)
];


pub enum CharOrStr<'l> {
    Char(char),
    Str(&'l str)
}
impl<'l> FromIterator<CharOrStr<'l>> for String {
    fn from_iter<T : IntoIterator<Item = CharOrStr<'l>>>(iter : T) -> Self {
        let mut s = String::new();
        for v in iter { match (v) {
            CharOrStr::Char(x) => { s.push(x); },
            CharOrStr::Str(x) => { s.push_str(x); },
        } }
        s
    }
}

pub fn make_pascalcase_ident(s : &str) -> String {
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


pub enum CharOrToLower {
    None,
    Char(char),
    ToLower(ToLowercase)
}
impl FromIterator<CharOrToLower> for String {
    fn from_iter<T : IntoIterator<Item = CharOrToLower>>(iter : T) -> Self {
        let mut s = String::new();
        for v in iter { match (v) {
            CharOrToLower::None => { },
            CharOrToLower::Char(x) => { s.push(x); },
            CharOrToLower::ToLower(x) => { _ = write!(s, "{}", x); },
        } }
        s
    }
}

pub fn make_snakecase_ident(s : &str) -> String {
    make_pascalcase_ident(s).chars()
        .flat_map(|ch| {
            if (ch.is_uppercase()) {
                [CharOrToLower::Char('_'), CharOrToLower::ToLower(ch.to_lowercase()),]
            } else {
                [CharOrToLower::None, CharOrToLower::Char(ch),]
            }
        })
        .skip_while(|ch| matches!(ch, CharOrToLower::Char('_')))
        .collect::<String>()
}


pub fn rhash_ident(s : &str) -> &str {
    if (s == "type") {
        "r#type"
    } else {
        s
    }
}
