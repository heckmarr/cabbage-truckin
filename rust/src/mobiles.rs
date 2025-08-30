use godot::prelude::*;
use godot::obj::Gd;
use godot::obj::NewAlloc;

use godot::classes::Timer;

#[derive(Debug, GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum MobileKind {
	Package,
	WarehousePerson,
	Cashier,
	Chef,
	Customer,
	Stocker,
}
use godot::classes::Node2D;
use godot::classes::INode2D;
use godot::classes::AudioStreamPlayer;
use godot::classes::AudioStreamWav;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Mobiles {
	drawing_arc: bool,
	arc_length: f32,
	hitpoints: i32,
	timer: Gd<Timer>,
	#[export]
	mob: MobileKind,
	base: Base<Node2D>,
}


#[godot_api]
impl Mobiles {
	#[signal]
	fn play_sound();
	#[signal]
	pub fn possessed();
	#[signal]
	fn balete();
	#[signal]
	fn mobile_random_damage_taken(amount: i32);
	#[signal]
	fn mobile_damage_taken(amount: i32);
	#[func]
	fn mobile_damage_emit(&mut self, amount: i32) {
		self.signals().mobile_damage_taken().emit(amount);
	}
	#[func]
	fn on_possess(&mut self) {
		self.drawing_arc = true;
		self.arc_length = 1.57;
		let name = self.base().get_name();
		godot_print!("Setting arc for {name} to {0}", self.drawing_arc);
	}
	#[func]
	fn mobile_random_damage_emit(&mut self) {
		//The random number range is 100-500
		let rand_num = randi_range(100, 500) as i32;
		self.signals().mobile_damage_taken().emit(rand_num);
	}
	fn on_mobile_damage_taken(&mut self, amount: i32) {
		self.hitpoints -= amount;
		let mut hp = self.hitpoints;
		//stop at zero! He's dead already!
		if hp <= 0 {
			hp = 0;
			self.signals().balete().emit();
		}
		godot_print!("packaging taking {amount} damage of {hp} total");
	}
	fn on_play_sound(&mut self) {
		let s_p = self.base().find_child("Noise").expect("No sound player for this object!");
		let sound_path = s_p.get_path();
		let mut sound_p: Gd<AudioStreamPlayer> = s_p.get_node_as(&sound_path);

		sound_p.play();

	}

}
use godot::global::randi_range;

#[godot_api]
impl INode2D for Mobiles {
	fn init(base: Base<Node2D>) -> Self {
		godot_print!("Mobile ready");
		Self {
			drawing_arc: false,
			arc_length: 1.57,
			hitpoints: 100,
			timer: Timer::new_alloc(),
			mob: MobileKind::Customer,
			base,
		}
	}
	fn ready(&mut self)  {
		let s_p = self.base().find_child("Noise").expect("No sound player for this object!");
		let sound_path = s_p.get_path();
		let mut sound_p: Gd<AudioStreamPlayer> = s_p.get_node_as(&sound_path);
		
		let sound = AudioStreamWav::load_from_file("res://audio/card-flthbp.wav").expect("No sound file!");
		sound_p.set_stream(&sound);
		self.signals().play_sound().connect_self(Self::on_play_sound);
		godot_print!("Connecting sounds for mobiles");


		godot_print!("Connecting signals for mobiles");
		self.signals().possessed().connect_self(Self::on_possess);
		self.signals()
			.mobile_damage_taken()
			.connect_self(Self::on_mobile_damage_taken);
		self.signals()
			.mobile_random_damage_taken()
			.connect_self(Self::on_mobile_damage_taken);
		match self.mob {
			MobileKind::Chef => {godot_print!("Chef!");
					self.base_mut().set_position(Vector2::new(300.0, 200.0));
					let pos = self.base().get_position();
					let mob_name = self.base().get_name();
					godot_print!("{mob_name} position moving to x: {pos}");
			}
			MobileKind::Customer => {godot_print!("Customer!");
					self.base_mut().set_position(Vector2::new(300.0, 200.0));
					let pos = self.base().get_position();
					let mob_name = self.base().get_name();
					godot_print!("{mob_name} position moving to x: {pos}");
			}
			MobileKind::Stocker => {godot_print!("Stocker!");
					self.base_mut().set_position(Vector2::new(650.0, 350.0));
					let pos = self.base().get_position();
					let mob_name = self.base().get_name();
					godot_print!("{mob_name} position moving to x: {pos}");
			}
			MobileKind::Cashier => {godot_print!("Cashier!");
					self.base_mut().set_position(Vector2::new(200.0, 400.0));
					let pos = self.base().get_position();
					let mob_name = self.base().get_name();
					godot_print!("{mob_name} position moving to x: {pos}");
			}
			MobileKind::Package => {godot_print!("Package!");
					self.base_mut().set_position(Vector2::new(800.0, 0.0));
					let pos = self.base().get_position();
					let mob_name = self.base().get_name();
					godot_print!("{mob_name} position moving to x: {pos}");
			}
			MobileKind::WarehousePerson => {godot_print!("WarehousePerson!");
					self.base_mut().set_position(Vector2::new(600.0, 0.0));
					let pos = self.base().get_position();
					let mob_name = self.base().get_name();
					godot_print!("{mob_name} position moving to x: {pos}");

			}
		}
	}
	fn process(&mut self, _delta: f32) {
		if self.drawing_arc {
                        self.arc_length = self.arc_length - 0.01745329;
                        if self.arc_length <= 0.0 {
                                self.drawing_arc = false;
                        }
                        self.base_mut().queue_redraw();
//                        godot_print!("arc length is {0}", self.arc_length);
                }

	}


	fn draw(&mut self) {
                if self.drawing_arc {
                        let col = Color::from_rgb(0.1, 1.0, 0.1);
			//Pos is relative to the sprite in this call, so we initialize it to Vector2(0, 0)
			let pos = Vector2::new(0.0, 0.0);

                        let arc_l = self.arc_length;
                        if self.arc_length <= 0.0 {
                                self.arc_length = 1.57;
                                self.drawing_arc = false;
                        }
//			godot_print!("Drawing arc!{0}", self.arc_length);
			self.signals().play_sound().emit();
                        let mut arc = self.base_mut();
                        arc.draw_arc_ex(pos, 100.0, 0.0, arc_l, 15, col).width(10.0).done();
                }
        }

}

impl Drop for Mobiles {
	fn drop(&mut self) {
		godot_print!("Dropping {0}", self.timer);
		self.timer.queue_free();
		//self.sound_player.queue_free();
	}
}
