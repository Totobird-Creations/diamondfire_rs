#![expect(unused_parens)]


use std::{
    collections::HashSet,
    fs::{ self, File },
    io::Write as _
};
use serde_json::from_reader as read_json;

mod ident;
use ident::identify;

mod actiondump;
pub use actiondump::ActionDump;


pub fn main() {
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
            if (! sound.icon.deprecation.is_empty()) {
                writeln!(f, "    #[deprecated]").unwrap();
            }
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
            if (! particle.icon.deprecation.is_empty()) {
                writeln!(f, "    #[deprecated]").unwrap();
            }
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
            if (! potion.icon.deprecation.is_empty()) {
                writeln!(f, "    #[deprecated]").unwrap();
            }
            writeln!(f, "    pub safe fn DF_POTION__{}(amplifier : df_number, duration : df_number) -> df_potion;", identify(&potion.icon.name)).unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

    {
        let mut f = File::create("src/generated/action.rs").unwrap();
        writeln!(f, "use crate::*;").unwrap();
        writeln!(f, "unsafe extern \"C\" {{").unwrap();
        for action in ad.actions {
            if (! action.icon.deprecation.is_empty()) {
                writeln!(f, "    #[deprecated]").unwrap();
            }
            write!(f, "    pub unsafe fn DF_ACTION__{}__{}", identify(&action.codeblock.to_lowercase()), identify(&action.name)).unwrap();
            for tag in &action.tags {
                write!(f, "__{}", identify(&tag.name)).unwrap();
            }
            write!(f, "(").unwrap();
            for tag in &action.tags {
                write!(f, "r#{} : df_string, ", tag.name.to_lowercase().replace(|ch| ! ident::VALID_CHARS.contains(&ch), "_")).unwrap();
            }
            writeln!(f, "...);").unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

}
