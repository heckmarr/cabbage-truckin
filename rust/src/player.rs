use godot::prelude::*;
use godot::classes::Sprite2D;
//ssss savoury for later!***********************
//use godot::global::randi_range;
use godot::obj::Gd;
use godot::classes::Timer;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
	arc_length: f32,
	draw_arc: bool,
	hitpoints: i32,
	cook_timer: Gd<Timer>,
	base: Base<Node2D>
}
//use crate::select::BoundRect;
use godot::classes::INode2D;


use godot::classes::Texture2D;
#[godot_api]
impl Player {
	//getter
	pub fn get_arc_length(&mut self) -> f32 {
		return self.arc_length;
	}
	//setter
	pub fn set_arc_length(&mut self, amount: f32) {
		self.arc_length = amount;
	}

	//setter
	pub fn set_draw_arc(&mut self, draw: bool) {
		self.draw_arc = draw;
	}

	#[signal]
	pub fn transform_the_boss();
	#[signal]
	pub fn boss_just_transformed();

	#[signal]
	fn damage_taken(amount: i32);
	#[func]
	fn on_timer_done() {
		godot_print!("Timer went off!");
	}
	#[func]
	fn damage_emit(&mut self, amount: i32) {
		self.signals().damage_taken().emit(amount);
	}
	#[func]
	fn on_transform_the_boss(&mut self) {
		let sprite = self.base().find_child("ghost_boss_spr").expect("No ghost boss sprite in tree!");
		let mut spr: Gd<Sprite2D> = self.base_mut().get_node_as(&sprite.get_path());
		let tex = load("res://sprites/ghost-boss-normal.png") as Gd<Texture2D>;
		spr.set_texture(&tex);
		self.draw_arc = false;
		//self.arc_length = 0.0;
		self.base_mut().queue_redraw();
	}
	#[func]
	fn on_boss_just_transformed(&mut self) {
		self.set_arc_length(1.57);
		let sprite = self.base().find_child("ghost_boss_spr").expect("No ghost boss sprite in tree!");
		let mut spr: Gd<Sprite2D> = self.base_mut().get_node_as(&sprite.get_path());
		let tex = load("res://sprites/ghost-boss-angry.png") as Gd<Texture2D>;
		spr.set_texture(&tex);
		if self.arc_length <= 0.0 {
			self.draw_arc = false;
			self.arc_length = 0.0;
		}
		self.draw_arc = true;
		self.base_mut().queue_redraw();
	}
	fn on_damage_taken(&mut self, amount: i32) {
		self.hitpoints -= amount;
		let mut hp = self.hitpoints;
		//stop at zero! He's dead already!
		if hp < 0 {
			hp = 0
		}
		godot_print!("Boss taking {amount} damage of {hp} total");
	}

	#[signal]
	fn balete();
}

#[godot_api]
impl INode2D for Player {
	fn init(base: Base<Node2D>) -> Self {
		godot_print!("Initializing Player"); //Prints to the godot console
		
		Self {
			arc_length: 1.57,
			draw_arc: false,
			hitpoints: 100,
			cook_timer: Timer::new_alloc(),
			base,
		}
	}
	fn process(&mut self, _delta: f32) {


		//nothing to see here!


	}

	fn draw(&mut self) {
		if self.draw_arc {
			let col = Color::from_rgb(0.1, 1.0, 0.1);
			let pos = self.base().get_position();
			let arc_l = self.arc_length;
			if self.arc_length <= 0.0 {
//				self.arc_length = 1.57;
				self.draw_arc = false;
			}

			let mut arc = self.base_mut();
			arc.draw_arc_ex(pos, 300.0, 0.0, arc_l, 15, col).width(100.0).done();
		}
	}

	fn ready(&mut self) { 

		let timer = self.base().get_tree().create_timer(5.0);
		timer.signals().timeout().connect(Player::on_timer_done);
		

		godot_print!("Putting the boss in the big chair");
		self.base_mut().set_position(Vector2::new(75.0, 100.0));
		self.signals()
			.boss_just_transformed()
			.connect_self(Player::on_boss_just_transformed);
		self.signals()
			.transform_the_boss()
			.connect_self(Player::on_transform_the_boss);
		godot_print!("Connecting signals for Boss"); 
		self.signals()
			.damage_taken()
			.connect_self(Player::on_damage_taken);
	}

}
impl Drop for Player {
	fn drop(&mut self) {
		self.cook_timer.queue_free();
		godot_print!("Thanks for playing Ghost Boss");
		//self.spr.queue_free();
	}
}
