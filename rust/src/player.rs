use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
	angular_speed: f32,
	speed: f32,
	position: Vector2,
	direction: f32,
	rotation: f32,
	base: Base<Sprite2D>
}
#[derive(GodotClass)]
#[class(base=Node2D)]
struct Mobile {
	hitpoints: i32,
	base: Base<Node2D>,
}
#[godot_api]
impl Mobile {
	#[signal]
	fn damage_taken(amount: i32);
	#[func]
	fn take_damage(&mut self, amount: i32) {
		self.hitpoints -= amount;
		let hp = self.hitpoints;
		godot_print!("Mobile taking {amount} damage of {hp} total");
	}
	#[func]
	fn hit_by_missile(&mut self, amount: i32) {
		self.hitpoints -= amount;
		let hp = self.hitpoints;
		godot_print!("Mobile hit by missile! {hp} left");
	}
}

#[godot_api]
impl INode2D for Mobile {
	fn init(base: Base<Node2D>) -> Self {
		Self {
			hitpoints: 200,
			base,
		}
	}

}

use godot::classes::ISprite2D;
use godot::classes::Input;
use godot::classes::InputEvent;
#[godot_api]
impl ISprite2D for Player {
	fn init(base: Base<Sprite2D>) -> Self {
		godot_print!("Hello, world!"); //Prints to the godot console

		Self {
			angular_speed: 3.14159,
			speed: 200.0,
			position: Vector2::ZERO,
			direction: 0.0,
			rotation: 0.0,
			base,
		}
	}
	fn process(&mut self, delta: f32) {
		self.rotation = self.base().get_rotation();
		self.direction = 0.0;
		let event = Input::singleton();
		let mut moved = false;
		let mut rot: f32 = self.rotation;
		if event.is_action_pressed("ui_left") {
			self.direction = -1.0;
			moved = true;
			self.rotation = (self.direction * self.angular_speed * delta) as f32;
			rot = self.rotation;
		}
		if event.is_action_pressed("ui_right") {
			self.direction = 1.0;
			moved = true;
			self.rotation = (self.direction * self.angular_speed * delta) as f32;
			rot = self.rotation;
		}
		if moved {
			self.base_mut().rotate(rot);
			moved = false;
		}
		let rot = self.base().get_rotation();		
//		self.position = Vector2::ZERO;
		if event.is_action_pressed("ui_up") {
			let velocity = Vector2::UP.rotated(rot) * self.speed as f32;
			self.position = velocity * delta;
			let pos: Vector2 = self.position;
			let in_bounds = self.base().get_position();
			if in_bounds.x <= 1100.0 && in_bounds.y <= 600.0 && in_bounds.x > 0.0 && in_bounds.y > 0.0{
				self.base_mut().translate(pos);
			}else {
				godot_print!("Hit boundary!");
			}
		}
		//let radians = (self.angular_speed * delta) as f32;
		//self.base_mut().rotate(radians);
		// .rotate() requires a f32, so we convert it as a f32 rather than f64
		//let rotation = self.base().get_rotation();
		//let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
		//self.base_mut().translate(velocity * delta as f32);
	}
}
