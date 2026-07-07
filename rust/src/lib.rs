use godot::prelude::*;

struct SpinnyBot;

#[gdextension]
unsafe impl ExtensionLibrary for SpinnyBot {}

mod mobiles;
mod select;
mod package;
mod gamestate;
mod player;
