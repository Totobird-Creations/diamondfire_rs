#![expect(unused_parens)]


use bridgecg_diamondfire::extern_names::{
    ExternNameMap,
    ExternName,
    ActionBlockKind
};
use std::{
    env,
    fs::{ self, File },
    io::Write
};
use serde_json::from_reader as read_json;

mod ident;
use ident::{
    make_pascalcase_ident,
    make_snakecase_ident,
    rhash_ident
};

mod actiondump;
use actiondump::ActionDump;


fn main() {
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    fs::create_dir_all("src/generated").unwrap();

    let ad = read_json::<_, ActionDump>(File::open("../actiondump.json").unwrap()).unwrap(); // TODO: Un-hardcode actiondump location.

    let mut extern_names = ExternNameMap::default();

    {
        let mut f = File::create("src/generated/gamevalue.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for gamevalue in ad.game_values {
            let ident = format!("DF_GAMEVALUE__{}", make_pascalcase_ident(&gamevalue.icon.name));
            if (extern_names.declare(ident.clone(), ExternName::Gamevalue {
                gamevalue : gamevalue.icon.name.clone()
            })) {
                write_attributes(&mut f, 4, &gamevalue.icon, None);
                writeln!(f, "    pub fn {}(target : *const df_string) -> {};", ident, gamevalue.icon.return_type.unwrap().type_name().unwrap()).unwrap();
            }
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/action.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for action in ad.actions.into_iter().rev() {
            if (action.codeblock == "PLAYER EVENT"
                || action.codeblock == "IF PLAYER"
                || action.codeblock == "ENTITY EVENT"
                || action.codeblock == "IF ENTITY"
                || action.codeblock == "IF VARIABLE"
                || action.codeblock == "IF GAME"
                || action.codeblock == "ELSE"
                || action.codeblock == "REPEAT"
                || action.codeblock == "FUNCTION"
                || action.codeblock == "CALL FUNCTION"
                || action.codeblock == "PROCESS"
                || action.codeblock == "START PROCESS"
            ) { continue; }
            let     codeblock    = ActionBlockKind::from(action.codeblock.as_str());
            let mut ident_action = make_pascalcase_ident(&action.name);
            if (ident_action == "x") { ident_action = String::from("Mul"); }
            let ident = format!("DF_ACTION__{:?}__{}",
                codeblock,
                ident_action
            );
            if (extern_names.declare(ident.clone(), ExternName::Action {
                codeblock,
                action       : action.name.clone(),
                tag_defaults : action.tags.iter().map(|tag| tag.name.clone()).collect::<Vec<_>>()
            })) {
                write_attributes(&mut f, 4, &action.icon, Some(&action.tags));
                write!(f, "    pub unsafe fn {}", ident).unwrap();
                write!(f, "(").unwrap();
                for tag in &action.tags {
                    write!(f, "{} : *const df_string, ", rhash_ident(&make_snakecase_ident(&tag.name))).unwrap();
                }
                write!(f, "...)").unwrap();
                if (action.codeblock == "CONTROL" && (action.name == "Return" || action.name == "ReturnNTimes" || action.name == "End")) {
                    write!(f, " -> !").unwrap();
                }
                writeln!(f, ";").unwrap();
            }
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/extern_names.bin").unwrap();
        extern_names.encode_write(&mut f).unwrap();
    }
    {
        let mut f = File::create("src/generated/extern_names.rs").unwrap();
        writeln!(f, "mod not_accessible_under_any_circumstance {{").unwrap();
        writeln!(f, "    #[unsafe(no_mangle)]").unwrap();
        writeln!(f, "    static __PRIVATE_DIAMONDFIRE_SYS__EXTERN_NAMES : &[u8] = include_bytes!(\"extern_names.bin\");").unwrap();
        writeln!(f, "}}").unwrap();
    }

}


fn write_attributes<W : Write>(f : &mut W, indent : usize,
    icon : &actiondump::ActionDumpIcon,
    tags : Option<&[actiondump::ActionDumpActionTag]>
) {
    let indent = " ".repeat(indent);

    if (! icon.name.is_empty()) {
        writeln!(f, "{indent}/// ## {}", escape_markdown(&icon.name)).unwrap();
    }
    if (! icon.description.is_empty()) {
        for line in &icon.description {
            writeln!(f, "{indent}/// {}", escape_markdown(line)).unwrap();
        }
    }
    writeln!(f, "{indent}///").unwrap();

    if let Some(tags) = tags && (! tags.is_empty()) {
        writeln!(f, "{indent}/// ### Tags").unwrap();
        for tag in tags {
            writeln!(f, "{indent}/// - {}:", escape_markdown(&tag.name)).unwrap();
            for option in &tag.options {
                write!(f, "{indent}///   - `{}`", option.name).unwrap();
                if (option.name == tag.default) {
                    write!(f, " (Default)").unwrap();
                }
                writeln!(f).unwrap();
            }
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.arguments.is_empty()) {
        writeln!(f, "{indent}/// ### Arguments").unwrap();
        for arg in &icon.arguments {
            let mut first = '-';
            if let Some(text) = &arg.text {
                writeln!(f, "{indent}/// {first} {text}").unwrap();
                first = ' ';
            }
            if let Some(kind) = &arg.kind {
                if let Some(name) = kind.type_name() {
                    write!(f, "{indent}/// {first} `{}`", name).unwrap();
                    if (arg.plural || arg.optional) {
                        write!(f, " `").unwrap();
                        if (arg.plural) {
                            write!(f, "[]").unwrap();
                        }
                        if (arg.optional) {
                            write!(f, "?").unwrap();
                        }
                        write!(f, "`").unwrap();
                    }
                    write!(f, " ({}):", kind.name()).unwrap();
                } else {
                    write!(f, "{indent}/// {first} None:").unwrap();
                }
                writeln!(f).unwrap();
                first = ' ';
            }
            for line in &arg.description {
                writeln!(f, "{indent}/// {first} {}", escape_markdown(line)).unwrap();
                first = ' ';
            }
            for group in &arg.notes {
                for (i, line,) in group.iter().enumerate() {
                    if (i == 0) {
                        writeln!(f, "{indent}///   - {}", escape_markdown(line)).unwrap();
                    } else {
                        writeln!(f, "{indent}///     {}", escape_markdown(line)).unwrap();
                    }
                }
            }
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.returns.is_empty() || ! icon.return_desc.is_empty()) {
        writeln!(f, "{indent}/// ### Returns").unwrap();

        for ret in &icon.returns {
            let mut first = '-';
            if let Some(text) = &ret.text {
                writeln!(f, "{indent}/// {first} {text}").unwrap();
                first = ' ';
            }
            if let Some(kind) = &ret.kind {
                if let Some(name) = kind.type_name() {
                    writeln!(f, "{indent}/// {first} `{}` ({}):", name, kind.name()).unwrap();
                } else {
                    writeln!(f, "{indent}/// {first} None:").unwrap();
                }
                first = ' ';
            }
            for line in &ret.description {
                writeln!(f, "{indent}/// {first} {}", escape_markdown(line)).unwrap();
                first = ' ';
            }
        }

        if let Some(ty) = &icon.return_type {
            writeln!(f, "{indent}/// `{}` ({}):", ty.type_name().unwrap(), ty.name()).unwrap();
        }

        for line in &icon.return_desc {
            writeln!(f, "{indent}/// {}", escape_markdown(line)).unwrap();
        }

        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.example.is_empty()) {
        writeln!(f, "{indent}/// ### Examples").unwrap();
        for line in &icon.example {
            writeln!(f, "{indent}/// - `{}`", escape_markdown(line)).unwrap();
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.works_with.is_empty()) {
        writeln!(f, "{indent}/// ### Works With").unwrap();
        for line in &icon.works_with {
            writeln!(f, "{indent}/// - `{}`", escape_markdown(line)).unwrap();
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.additional_info.is_empty()) {
        writeln!(f, "{indent}/// ### Additional Info").unwrap();
        for group in &icon.additional_info {
            for (i, line,) in group.iter().enumerate() {
                if (i == 0) {
                    writeln!(f, "{indent}/// - {}", escape_markdown(line)).unwrap();
                } else {
                    writeln!(f, "{indent}///   {}", escape_markdown(line)).unwrap();
                }
            }
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if ((! icon.required_rank.is_none()) || icon.required_tokens || icon.required_rank_and_tokens) {
        writeln!(f, "{indent}/// ### Restrictions").unwrap();
        if let Some(name) = icon.required_rank.name() {
            writeln!(f, "{indent}/// - Requires **{}** rank", name).unwrap();
        }
        if (icon.required_tokens) {
            writeln!(f, "{indent}/// - Requires token shop purchase").unwrap();
        }
        if (icon.required_rank_and_tokens) {
            writeln!(f, "{indent}/// - Requires rank and token shop purchase").unwrap();
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.deprecation.is_empty()) {
        writeln!(f, "{indent}#[deprecated = {:?}]", icon.deprecation.join(" ")).unwrap();
    } else if (icon.name.is_empty()) {
        writeln!(f, "{indent}#[deprecated]").unwrap();
    }

    if let Some(rank_feature) = icon.required_rank.feature_name() {
        writeln!(f, "{indent}#[cfg(any(doc, feature = {:?}))]", rank_feature).unwrap();
        writeln!(f, "{indent}#[cfg_attr(doc, doc(cfg(feature = {:?})))]", rank_feature).unwrap();
    }
}


fn escape_markdown(s : &str) -> String {
    s.replace('<', "\\<")
        .replace('>', "\\>")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('`', "\\`")
}
