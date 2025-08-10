use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
	speed: f64,
	position: Vector2,
	delta: f32,
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
			speed: 400.0,
			position: Vector2::ZERO,
			delta: 0.0,
			direction: 0.0,
			rotation: 0.0,
			base,
		}
	}
	fn process(&mut self, delta: f32) {
		self.delta = delta;
	}

	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if event.is_action_pressed("ui_left") {
			self.direction = -1.0;
		}
		if event.is_action_pressed("ui_right") {
			self.direction = 1.0;
		}
		self.rotation += self.direction as f32;
		let rot: f32 = self.rotation;
		self.base_mut().rotate(rot);		
		let mut velocity = Vector2::ZERO;
		if event.is_action_pressed("ui_up") {
			velocity = Vector2::UP.rotated(self.rotation) * self.speed as f32;
		}
		self.position += velocity * self.delta;
		let pos: Vector2 = self.position;
		self.base_mut().translate(pos);
		//let radians = (self.angular_speed * delta) as f32;
		//self.base_mut().rotate(radians);
		// .rotate() requires a f32, so we convert it as a f32 rather than f64
		//let rotation = self.base().get_rotation();
		//let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
		//self.base_mut().translate(velocity * delta as f32);
	}
}
