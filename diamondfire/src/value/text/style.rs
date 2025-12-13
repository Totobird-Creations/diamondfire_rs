use crate::value::Text;


pub trait Stylise {

    // TOOD: black

    // TODO: dark_blue

    // TODO: dark_green

    // TODO: dark_cyan

    // TODO: dark_red

    // TODO: purple

    // TODO: gold

    // TODO: grey

    // TODO: dark_grey

    // TODO: blue

    // TODO: green

    // TODO: cyan

    // TODO: red

    // TODO: pink

    // TODO: yellow

    // TODO: white

    // TODO: rgb

    // TODO: shadow

    // TODO: bold

    // TODO: italic

    // TODO: underline

    // TODO: strike

    // TODO: obfuscate

    // TODO: reset

    // TODO: click_event

    // TODO: hover_event

    // TODO: insertion

    // TODO: rainbow

    // TODO: gradient

    // TODO: transition

    // TODO: font

}


impl<T : Into<Text>> Stylise for T {}
