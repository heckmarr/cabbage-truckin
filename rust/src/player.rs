use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::AnimatedSprite2D;
use godot::global::randi_range;
use godot::obj::Gd;
use godot::classes::Timer;


#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
	hitpoints: i32,
	angular_speed: f32,
	speed: f32,
	position: Vector2,
	direction: f32,
	rotation: f32,
	anim: Gd<AnimatedSprite2D>,
	boss_timer: Gd<Timer>,
	base: Base<Sprite2D>
}


use godot::classes::ISprite2D;
use godot::classes::Input;
#[godot_api]
impl Player {
	#[signal]
	fn damage_all_mobiles(amount: i32);
	#[signal]
	fn random_damage_taken(amount: i32);
	#[signal]
	fn damage_taken(amount: i32);
	#[func]
	fn boop() {
		godot_print!("Timer went off!");
	}
	#[func]
	fn damage_emit(&mut self, amount: i32) {
		self.signals().damage_taken().emit(amount);
	}
	#[func]
	fn random_damage_emit(&mut self) {
		//The random number range is 100-500
		let rand_num = randi_range(100, 500) as i32;
		self.signals().random_damage_taken().emit(rand_num);
	}
	fn on_damage_taken(&mut self, amount: i32) {
		self.hitpoints -= amount;
		let mut hp = self.hitpoints;
		//stop at zero! He's dead already!
		if hp < 0 {
			hp = 0
		}
		godot_print!("Player taking {amount} damage of {hp} total");
	}
	#[signal]
	fn balete();
}

#[godot_api]
impl ISprite2D for Player {
	fn init(base: Base<Sprite2D>) -> Self {
		godot_print!("Initializing Player"); //Prints to the godot console

		Self {
			hitpoints: 100,
			angular_speed: 3.14159,
			speed: 200.0,
			position: Vector2::ZERO,
			direction: 0.0,
			rotation: 0.0,
			anim: AnimatedSprite2D::new_alloc(),
			boss_timer: Timer::new_alloc(),
			base,
		}
	}
	fn process(&mut self, delta: f32) {
		self.rotation = self.base().get_rotation();
		self.direction = 0.0;
		let event = Input::singleton();
		let mut moved = false;
		let mut rot: f32 = self.rotation;

		if event.is_action_just_pressed("Pad-A") {
			self.damage_emit(50);
		}
		if event.is_action_just_pressed("Pad-B") {
			self.random_damage_emit();
		}
		if event.is_action_just_pressed("Pad-Y") {
			godot_print!("Y");
			self.signals().damage_all_mobiles().emit(100);
		}

		if event.is_action_just_pressed("Pad-X") {
			godot_print!("X");
			self.signals().balete().emit();
		}
		if event.is_action_pressed("ui_left") {
			godot_print!("playing {0}", self.anim);
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
		}
		let rot = self.base().get_rotation();		
		if event.is_action_pressed("ui_up") {
			let velocity = Vector2::UP.rotated(rot) * self.speed as f32;
			self.position = velocity * delta;
			let pos: Vector2 = self.position;
			let in_bounds = self.base().get_position();
			if in_bounds.x <= 1100.0 && in_bounds.y <= 600.0 && in_bounds.x > 0.0 && in_bounds.y > 0.0{
				self.base_mut().translate(pos);
			}else {
				godot_print!("Player hit boundary!");
			}
		}
	}

	fn ready(&mut self) { 
	
		self.boss_timer.set_wait_time(5.0);
		self.boss_timer.set_autostart(true);
		self.boss_timer.set_one_shot(true);
		self.boss_timer.signals().timeout().connect(Player::boop);
		let timing = self.boss_timer.clone();
		self.base_mut().add_child(&timing);
		godot_print!("Connecting signals for Player"); 
		self.signals()
			.damage_taken()
			.connect_self(Player::on_damage_taken);
		self.signals()
			.random_damage_taken()
			.connect_self(Player::on_damage_taken);
	}

}
impl Drop for Player {
	fn drop(&mut self) {
		godot_print!("dropping {0}", self.anim);
		self.anim.queue_free();
		self.boss_timer.queue_free();
	}
}
