use godot::prelude::*;

struct SpinnyBot;

#[gdextension]
unsafe impl ExtensionLibrary for SpinnyBot {}

mod player;
