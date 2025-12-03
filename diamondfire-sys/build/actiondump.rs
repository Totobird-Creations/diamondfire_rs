use serde::{
    Deserialize as Deser,
    de::IgnoredAny
};


#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDump {
    #[serde(rename = "codeblocks")]
    pub code_blocks   : Vec<ActionDumpCodeBlock>,
    pub actions       : Vec<ActionDumpAction>,
    #[serde(rename = "gameValueCategories")]
    pub game_val_cats : IgnoredAny,
    #[serde(rename = "gameValues")]
    pub game_values   : Vec<ActionDumpGameValue>,
    #[serde(rename = "particleCategories")]
    pub particle_cats : IgnoredAny,
    pub particles     : Vec<ActionDumpParticle>,
    #[serde(rename = "soundCategories")]
    pub sound_cats    : IgnoredAny,
    pub sounds        : Vec<ActionDumpSound>,
    pub potions       : Vec<ActionDumpPotion>,
    pub cosmetics     : IgnoredAny,
    pub shops         : IgnoredAny
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpCodeBlock {
    pub name  : String,
    #[serde(rename = "identifier")]
    pub ident : String,
    pub item  : ActionDumpIcon
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpIcon {
    pub material                 : String,
    pub name                     : String,
    #[serde(default)]
    pub head                     : Option<String>,
    #[serde(default, rename = "color")]
    pub colour                   : Option<ActionDumpColour>,
    #[serde(rename = "deprecatedNote")]
    pub deprecation              : Vec<String>,
    #[serde(rename = "description")]
    pub description              : Vec<String>,
    #[serde(rename = "example")]
    pub example                  : Vec<String>,
    #[serde(rename = "worksWith")]
    pub works_with               : Vec<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info          : Vec<Vec<String>>,
    #[serde(rename = "requiredRank")]
    pub required_rank            : ActionDumpRequiredRank,
    #[serde(rename = "requireTokens")]
    pub required_tokens          : bool,
    #[serde(rename = "requireRankAndTokens")]
    pub required_rank_and_tokens : bool,
    pub advanced                 : bool,
    #[serde(rename = "loadedItem")]
    pub loaded_item              : String,
    #[serde(default)]
    pub cancellable              : Option<bool>,
    #[serde(default, rename = "cancelledAutomatically")]
    pub cancelled_automatically  : Option<bool>,
    #[serde(default)]
    pub tags                     : u8,
    #[serde(default)]
    pub arguments                : Vec<ActionDumpIconArgument>,
    #[serde(default, rename = "returnValues")]
    pub returns                  : Vec<ActionDumpIconReturn>,
    #[serde(default, rename = "returnType")]
    pub return_type              : Option<ActionDumpValueType>,
    #[serde(default, rename = "returnDescription")]
    pub return_desc              : Vec<String>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpColour {
    pub red   : u8,
    pub green : u8,
    pub blue  : u8
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub enum ActionDumpRequiredRank {
    #[serde(rename = "")]
    None,
    Noble,
    Emperor,
    Mythic,
    Overlord,
    Dev
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpAction {
    pub name        : String,
    #[serde(rename = "codeblockName")]
    pub codeblock   : String,
    pub tags        : Vec<ActionDumpActionTag>,
    pub aliases     : Vec<String>,
    pub icon        : ActionDumpIcon,
    #[serde(default, rename = "subActionBlocks")]
    pub sub_actions : Vec<String>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpActionTag {
    pub name    : String,
    pub options : Vec<ActionDumpActionTagOption>,
    #[serde(rename = "defaultOption")]
    pub default : String,
    pub slot    : u8
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpActionTagOption {
    pub name    : String,
    pub icon    : ActionDumpIcon,
    pub aliases : Vec<String>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpIconArgument {
    #[serde(default, rename = "type")]
    pub kind        : Option<ActionDumpValueType>,
    #[serde(default)]
    pub plural      : bool,
    #[serde(default)]
    pub optional    : bool,
    #[serde(default)]
    pub description : Vec<String>,
    #[serde(default)]
    pub notes       : Vec<Vec<String>>,
    #[serde(default)]
    pub text        : Option<String>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpIconReturn {
    #[serde(default, rename = "type")]
    pub kind        : Option<ActionDumpValueType>,
    #[serde(default)]
    pub description : Vec<String>,
    #[serde(default)]
    pub text        : Option<String>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub enum ActionDumpValueType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ANY_TYPE")]
    Any,
    #[serde(rename = "TEXT", alias = "BLOCK_TAG")]
    String,
    #[serde(rename = "COMPONENT")]
    Text,
    #[serde(rename = "NUMBER", alias = "BYTE")]
    Number,
    #[serde(rename = "LOCATION")]
    Location,
    #[serde(rename = "VECTOR")]
    Vector,
    #[serde(rename = "SOUND")]
    Sound,
    #[serde(rename = "PARTICLE")]
    Particle,
    #[serde(rename = "POTION")]
    Potion,
    #[serde(rename = "ITEM", alias = "BLOCK", alias = "PROJECTILE", alias = "SPAWN_EGG", alias = "ENTITY_TYPE", alias = "VEHICLE")]
    Item,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "DICT")]
    Dict,
    #[serde(rename = "VARIABLE")]
    Variable
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpGameValue {
    pub aliases  : Vec<String>,
    pub category : String,
    pub icon     : ActionDumpIcon
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpParticle {
    pub particle : String,
    pub icon     : ActionDumpIcon,
    #[serde(default)]
    pub category : Option<String>,
    pub fields   : Vec<ActionDumpParticleField>
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deser, Debug)]
#[serde(deny_unknown_fields)]
pub enum ActionDumpParticleField {
    Motion,
    Material,
    #[serde(rename = "Color")]
    Colour,
    #[serde(rename = "Fade Color")]
    FadeColour,
    Opacity,
    Size,
    Roll,
    Duration,
    #[serde(rename = "Motion Variation")]
    MotionVariation,
    #[serde(rename = "Color Variation")]
    ColourVariation,
    #[serde(rename = "Size Variation")]
    SizeVariation
}
impl ActionDumpParticleField {
    pub fn camel_name(self) -> &'static str { match (self) {
        Self::Motion          => "motion",
        Self::Material        => "material",
        Self::Colour          => "colour",
        Self::FadeColour      => "fade_colour",
        Self::Opacity         => "opacity",
        Self::Size            => "size",
        Self::Roll            => "roll",
        Self::Duration        => "duration",
        Self::MotionVariation => "motion_variation",
        Self::ColourVariation => "colour_variation",
        Self::SizeVariation   => "size_variation"
    } }
    pub fn type_name(self) -> &'static str { match (self) {
        Self::Motion          => "df_vector",
        Self::Material        => "df_string",
        Self::Colour          => "df_string",
        Self::FadeColour      => "df_string",
        Self::Opacity         => "df_number",
        Self::Size            => "df_number",
        Self::Roll            => "df_number",
        Self::Duration        => "df_number",
        Self::MotionVariation => "df_number",
        Self::ColourVariation => "df_number",
        Self::SizeVariation   => "df_number"
    } }
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpSound {
    pub sound    : String,
    #[serde(rename = "soundId")]
    pub sound_id : String,
    pub icon     : ActionDumpIcon,
    #[serde(default)]
    pub variants : Vec<ActionDumpSoundVariant>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpSoundVariant {
    pub id   : String,
    pub name : String,
    pub seed : i64
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionDumpPotion {
    pub potion : String,
    pub icon   : ActionDumpIcon
}
