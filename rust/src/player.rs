use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::AnimatedSprite2D;
use godot::global::randi_range;
use godot::obj::Gd;
use godot::classes::Timer;
use godot::classes::Node2D;



#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
	chosen: i32,
	selected_mob: i32,
	hitpoints: i32,
	direction: i32,
	spr: Gd<Sprite2D>,
	anim: Gd<AnimatedSprite2D>,
	boss_timer: Gd<Timer>,
	tex: Gd<Texture2D>,
	base: Base<Node2D>
}
use crate::mobiles::Mobiles;
use crate::targ::BoundRect;

use godot::classes::INode2D;


use godot::classes::ISprite2D;
use godot::classes::Input;
use godot::classes::Texture2D;
#[godot_api]
impl Player {
	#[signal]
	fn unboop_the_boss();
	#[signal]
	fn boop_the_boss();
	#[signal]
	fn damage_all_mobiles(amount: i32);
	#[signal]
	fn random_damage_taken(amount: i32);
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
	fn random_damage_emit(&mut self) {
		//The random number range is 100-500
		let rand_num = randi_range(100, 500) as i32;
		self.signals().random_damage_taken().emit(rand_num);
	}
	#[func]
	fn on_unboop_the_boss(&mut self) {
		self.tex = load("res://sprites/ghost-boss-normal.png") as Gd<Texture2D>;
		self.spr.set_texture(&self.tex);
	}
	#[func]
	fn on_boop_the_boss(&mut self)  {
		self.tex = load("res://sprites/ghost-boss-angry.png") as Gd<Texture2D>;
		self.spr.set_texture(&self.tex);
		
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
impl INode2D for Player {
	fn init(base: Base<Node2D>) -> Self {
		godot_print!("Initializing Player"); //Prints to the godot console

		Self {
			spr: Sprite2D::new_alloc(),
			selected_mob: 0,
			chosen: 0,
			hitpoints: 100,
			direction: 0,
			anim: AnimatedSprite2D::new_alloc(),
			boss_timer: Timer::new_alloc(),
			tex: Texture2D::new_gd(),
			base,
		}
	}
	fn process(&mut self, _delta: f32) {
		self.direction = 0;
		let event = Input::singleton();

		if event.is_action_just_pressed("Pad-A") {
			self.damage_emit(50);
		}
		if event.is_action_pressed("Pad-B") {
			self.signals().boop_the_boss().emit();
		}
		if event.is_action_just_released("Pad-B") {
			self.signals().unboop_the_boss().emit();
		}
		if event.is_action_just_pressed("Pad-Y") {
			godot_print!("Y");
			self.signals().damage_all_mobiles().emit(100);
		}

		if event.is_action_just_pressed("Pad-X") {
			godot_print!("X");
			self.signals().balete().emit();
		}
		if event.is_action_just_pressed("ui_left") {
			godot_print!("moving selection left");
			self.direction = -1;
			self.chosen = self.chosen + self.direction;
		}
		if event.is_action_just_pressed("ui_right") {
			godot_print!("Moving selection to the right");
			self.direction = 1;
			self.chosen = self.chosen + self.direction;
		}
		if self.chosen < 0 {
			self.chosen = 3;
		}else if self.chosen > 3 {
			self.chosen = 0;
		}
		let mut types_of_mob = Array::new();
		types_of_mob.push(0);
		types_of_mob.push(1);
		types_of_mob.push(2);
		types_of_mob.push(3);
		self.selected_mob = types_of_mob.at(self.chosen.try_into().unwrap());
		let br: Gd<Node> = self.base_mut().find_child("BoundRect").expect("No BoundRect in scene!");
		let br_path = br.get_path();
		let mut br_obj: Gd<BoundRect> = br.get_node_as(&br_path);
		let pos = br_obj.get_position();
		match self.selected_mob {
                        0 => {godot_print!("Chef!");
				let  mob: Gd<Node> = self.base().find_child("Chef").expect("No chef in tree!");
				let mob_path = mob.get_path();
				let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
				//Now set the position
				br_obj.set_position(mob_obj.get_position());
				let mob_name = mob.get_name();
                                godot_print!("{mob_name} at {mob_path} being selected at {pos}");
       	                }
                        1 => {godot_print!("Stocker!");
				let  mob: Gd<Node> = self.base().find_child("Stocker").expect("No stocker in tree!");
				let mob_path = mob.get_path();
				let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
				//Now set the position
				br_obj.set_position(mob_obj.get_position());
				let mob_name = mob.get_name();
                                godot_print!("{mob_name} at {mob_path} being selected at {pos}");

			}
                        2 => {godot_print!("Cashier!");
				let  mob: Gd<Node> = self.base().find_child("Cashier").expect("No cashier in tree!");
				let mob_path = mob.get_path();
				let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
				//Now set the position
				br_obj.set_position(mob_obj.get_position());
				let mob_name = mob.get_name();
                                godot_print!("{mob_name} at {mob_path} being selected at {pos}");

			}
                        3 => {godot_print!("WarehousePerson!");
				let  mob: Gd<Node> = self.base().find_child("WarehousePerson").expect("No warehouseperson in tree!");
				let mob_path = mob.get_path();
				let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
				//Now set the position
				br_obj.set_position(mob_obj.get_position());
				let mob_name = mob.get_name();
                                godot_print!("{mob_name} at {mob_path} being selected at {pos}");

			}
			_ => {godot_print!("Customer or other unselectable");}
		}
	}

	fn ready(&mut self) { 
		self.signals()
			.unboop_the_boss()
			.connect_self(Player::on_unboop_the_boss);
		self.signals()
			.boop_the_boss()
			.connect_self(Player::on_boop_the_boss);
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
