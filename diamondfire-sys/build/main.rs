#![expect(unused_parens)]


use std::{
    collections::HashSet,
    env,
    fs::{ self, File },
    io::Write
};
use serde_json::from_reader as read_json;

mod ident;
use ident::identify;

mod actiondump;
pub use actiondump::ActionDump;


pub fn main() {
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    fs::create_dir_all("src/generated").unwrap();

    let ad = read_json::<_, ActionDump>(File::open("../actiondump.json").unwrap()).unwrap();

    {
        let mut taken_names = HashSet::new();
        let mut f = File::create("src/generated/sound.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for sound in ad.sounds {
            let name = identify(&sound.icon.name);
            if (taken_names.contains(&name)) { continue; }
            write_attributes(&mut f, 4, &sound.icon, None);
            writeln!(f, "    pub safe fn DF_SOUND__{}(pitch : df_number, volume : df_number) -> df_sound;", name).unwrap();
            for variant in sound.variants {
                writeln!(f, "    pub safe fn DF_SOUND__{}__{}(pitch : df_number, volume : df_number) -> df_sound;", name, identify(&variant.id)).unwrap();
            }
            taken_names.insert(name);
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/particle.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for mut particle in ad.particles {
            write_attributes(&mut f, 4, &particle.icon, None);
            write!(f, "    pub safe fn DF_PARTICLE__{}__", identify(&particle.particle.to_lowercase())).unwrap();
            particle.fields.sort();
            for (i, field,) in particle.fields.iter().enumerate() {
                if (i > 0) { write!(f, "_").unwrap(); }
                write!(f, "{:?}", field).unwrap();
            }
            write!(f, "(amount : df_number, spread_x : df_number, spread_y : df_number").unwrap();
            for field in &particle.fields {
                write!(f, ", {} : {}", field.camel_name(), field.type_name()).unwrap();
            }
            writeln!(f, ") -> df_particle;").unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/potion.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for potion in ad.potions {
            write_attributes(&mut f, 4, &potion.icon, None);
            writeln!(f, "    pub safe fn DF_POTION__{}(amplifier : df_number, duration : df_number) -> df_potion;", identify(&potion.icon.name)).unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/gamevalue.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for gamevalue in ad.game_values {
            write_attributes(&mut f, 4, &gamevalue.icon, None);
            writeln!(f, "    pub safe fn DF_GAMEVALUE__{}(target : df_string) -> {};", identify(&gamevalue.icon.name), gamevalue.icon.return_type.unwrap().type_name().unwrap()).unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/action.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for action in ad.actions {
            if (action.codeblock == "PLAYER EVENT" || action.codeblock == "ENTITY EVENT" || action.codeblock == "FUNCTION" || action.codeblock == "CALL FUNCTION" || action.codeblock == "PROCESS" || action.codeblock == "START PROCESS") { continue; }
            write_attributes(&mut f, 4, &action.icon, Some(&action.tags));
            write!(f, "    pub unsafe fn DF_ACTION__{}__{}", identify(&action.codeblock.to_lowercase()), identify(&action.name)).unwrap();
            write!(f, "(").unwrap();
            for tag in &action.tags {
                write!(f, "{} : df_string, ", identify(&tag.name)).unwrap();
            }
            write!(f, "...)").unwrap();
            if (action.codeblock == "CONTROL" && (action.name == "Return" || action.name == "ReturnNTimes" || action.name == "End")) {
                write!(f, " -> !").unwrap();
            }
            writeln!(f, ";").unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

}


fn write_attributes<W : Write>(f : &mut W, indent : usize,
    icon : &actiondump::ActionDumpIcon,
    tags : Option<&[actiondump::ActionDumpActionTag]>
) {
    let indent = " ".repeat(indent);

    if (! icon.name.is_empty()) {
        writeln!(f, "{indent}/// **{}**<br/>", escape_markdown(&icon.name)).unwrap();
    }
    if (! icon.description.is_empty()) {
        for line in &icon.description {
            writeln!(f, "{indent}/// {}", escape_markdown(line)).unwrap();
        }
    }
    writeln!(f, "{indent}///").unwrap();

    if let Some(tags) = tags && (! tags.is_empty()) {
        writeln!(f, "{indent}/// ## Tags").unwrap();
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
        writeln!(f, "{indent}/// ## Arguments").unwrap();
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
        writeln!(f, "{indent}/// ## Returns").unwrap();

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
        writeln!(f, "{indent}/// ## Examples").unwrap();
        for line in &icon.example {
            writeln!(f, "{indent}/// - `{}`", escape_markdown(line)).unwrap();
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.works_with.is_empty()) {
        writeln!(f, "{indent}/// ## Works With").unwrap();
        for line in &icon.works_with {
            writeln!(f, "{indent}/// - `{}`", escape_markdown(line)).unwrap();
        }
        writeln!(f, "{indent}///").unwrap();
    }

    if (! icon.additional_info.is_empty()) {
        writeln!(f, "{indent}/// ## Additional Info").unwrap();
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
        writeln!(f, "{indent}/// ## Restrictions").unwrap();
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
