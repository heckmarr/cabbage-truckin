use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
	speed: f64,
	angular_speed: f64,

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

#[godot_api]
impl ISprite2D for Player {
	fn init(base: Base<Sprite2D>) -> Self {
		godot_print!("Hello, world!"); //Prints to the godot console

		Self {
			speed: 400.0,
			angular_speed: std::f64::consts::PI,
			base,
		}
	}

	fn physics_process(&mut self, delta: f64) {
		let radians = (self.angular_speed * delta) as f32;
		self.base_mut().rotate(radians);
		// .rotate() requires a f32, so we convert it as a f32 rather than f64
		let rotation = self.base().get_rotation();
		let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
		self.base_mut().translate(velocity * delta as f32);
	}
}
