use godot::prelude::*;
use crate::mobiles::MobileKind;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Background {
	x_pos: i32,
	y_pos: i32,
	visible: bool,
	#[export]
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
		//Position all sprites
	
		match self.mob {
                        MobileKind::Chef => {//godot_print!("Chefs table");
				let nam: String = self.base().get_name().to_string();
				let nam_str: &str = nam.as_str();
				match nam_str {
					"01" =>
					{
						godot_print!("Positioning part 01chef");
						self.base_mut().set_position(Vector2::new(200.0, 125.0));
					}
					"02" =>
					{
						godot_print!("Positioning part 02chef");
						self.base_mut().set_position(Vector2::new(350.0, 125.0));
					}
					"03" =>
					{
						godot_print!("Positioning part 03chef");
						self.base_mut().set_position(Vector2::new(500.0, 125.0));
					}
					"04" =>
					{	
						godot_print!("Positioning part 04chef");
						self.base_mut().set_position(Vector2::new(650.0, 125.0));
					}
					&_ => todo!()
				}
                        }
                        MobileKind::Customer => {godot_print!("Customer!");
                        }
                        MobileKind::Stocker => {godot_print!("Stocker!");
                        }
                        MobileKind::Cashier => {godot_print!("Cashier!");
                        }
                        MobileKind::Package => {godot_print!("Package!");
                        }
                        MobileKind::WarehousePerson => {godot_print!("WarehousePerson!");

        	                }
	                }
	
	}
}
