use godot::prelude::*;
use crate::mobiles::MobileKind;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Background {
	x_pos: i32,
	y_pos: i32,
	visible: bool,
	mob: MobileKind,
	base: Base<Sprite2D>,
}

use godot::classes::Sprite2D;
use godot::classes::ISprite2D;

#[godot_api]
impl Background {

	fn animate(&mut self) {
		//do thingy
	}
}

#[godot_api]
impl ISprite2D for Background {

	fn init(base: Base<Sprite2D>) -> Self {
		godot_print!("Background ready");
		Self {
			x_pos: 0,
			y_pos: 0,
			visible: false,
			mob: MobileKind::Customer,
			base,
		}
	}

	fn ready(&mut self) {
		godot_print!("Position all sprites");
		
	}
}
