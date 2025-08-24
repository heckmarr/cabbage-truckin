use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::AnimatedSprite2D;
use godot::global::randi_range;
use godot::obj::Gd;
use godot::classes::Timer;
use godot::classes::Node2D;

use crate::mobiles::MobileKind;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
	chosen_mob: MobileKind,
	chosen: i32,
	arc_length: f32,
	draw_arc: bool,
	hitpoints: i32,
	direction: i32,
	spr: Gd<Sprite2D>,
	anim: Gd<AnimatedSprite2D>,
	cook_timer: Gd<Timer>,
	tex: Gd<Texture2D>,
	base: Base<Node2D>
}
use crate::mobiles::Mobiles;
use crate::select::BoundRect;

use godot::classes::INode2D;

use godot::classes::Input;
use godot::classes::Texture2D;
#[godot_api]
impl Player {
	#[signal]
	fn unboop_the_boss();
	#[signal]
	fn boop_the_boss();
	#[signal]
	fn boss_just_booped();
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
		self.draw_arc = false;
		self.arc_length = 0.0;
	}
	#[func]
	fn on_boss_just_booped(&mut self) {
		self.arc_length = 1.57;
	}
	#[func]
	fn on_boop_the_boss(&mut self)  {
		self.tex = load("res://sprites/ghost-boss-angry.png") as Gd<Texture2D>;
		self.spr.set_texture(&self.tex);
		self.draw_arc = true;
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
			chosen_mob: MobileKind::Customer,
			arc_length: 1.57,
			draw_arc: false,
			spr: Sprite2D::new_alloc(),
			chosen: 0,
			hitpoints: 100,
			direction: 0,
			anim: AnimatedSprite2D::new_alloc(),
			cook_timer: Timer::new_alloc(),
			tex: Texture2D::new_gd(),
			base,
		}
	}
	fn process(&mut self, _delta: f32) {


		self.direction = 0;
		let event = Input::singleton();

		if event.is_action_just_pressed("ui_cancel") {
			self.base().get_tree().expect("Not in a tree!").quit();
		}

		if event.is_action_just_pressed("Pad-A") || event.is_action_just_pressed("ui_select") {
			self.damage_emit(50);
			//Possess the selected object
			match self.chosen_mob {
	                        MobileKind::Chef => {

					let mob = self.base().find_child("Chef").expect("Mob not chosen!");
					let mob_path = mob.get_path();
					let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
					mob_obj.signals().possessed().emit();
					godot_print!("{mob} being possessed!");
  	      	                }
                        	MobileKind::Stocker => {
					let mob = self.base().find_child("Stocker").expect("Mob not chosen!");
					let mob_path = mob.get_path();
					let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
					mob_obj.signals().possessed().emit();
					godot_print!("{mob} being possessed!");

                        	}
                        	MobileKind::Cashier => {
					let mob = self.base().find_child("Cashier").expect("Mob not chosen!");
					let mob_path = mob.get_path();
					let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
					mob_obj.signals().possessed().emit();
					godot_print!("{mob} being possessed!");

                        	}
                        	MobileKind::WarehousePerson => {
					let mob = self.base().find_child("WarehousePerson").expect("Mob not chosen!");
					let mob_path = mob.get_path();
					let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
					mob_obj.signals().possessed().emit();
					godot_print!("{mob} being possessed!");

                        	}
				_ => {
					//pass
				}

			}
		}
		if event.is_action_just_pressed("ui_select") {
			self.signals().boss_just_booped().emit();
		}
		if event.is_action_just_released("ui_select") {
			self.signals().unboop_the_boss().emit();
		}
		if event.is_action_pressed("ui_select") {
			self.signals().boop_the_boss().emit();
			
		}
		if event.is_action_just_pressed("Pad-B") {
			self.signals().boss_just_booped().emit();
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
		{//match scope
			let mut print = true;
			if self.direction == 0 {
				print = false;
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
			if print {
				let selected_mob = types_of_mob.at(self.chosen.try_into().unwrap());
				let br: Gd<Node> = self.base_mut().find_child("BoundRect").expect("No BoundRect in scene!");
				let br_path = br.get_path();
				let mut br_obj: Gd<BoundRect> = br.get_node_as(&br_path);
				let pos = br_obj.get_position();
				br_obj.set_visible(true);
				match selected_mob {
        	        	        0 => {		
						self.chosen_mob = MobileKind::Chef;
						let  mob: Gd<Node> = self.base().find_child("Chef").expect("No chef in tree!");
						let mob_path = mob.get_path();
						let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
						//Now set the position
						br_obj.set_position(mob_obj.get_position());
						let mob_name = mob.get_name();
						godot_print!("{mob_name} at {mob_path} being selected at {pos}");
					
	       	                	}
        	                	1 => {	
						self.chosen_mob = MobileKind::Stocker;
						let  mob: Gd<Node> = self.base().find_child("Stocker").expect("No stocker in tree!");
						let mob_path = mob.get_path();
						let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
						//Now set the position
						br_obj.set_position(mob_obj.get_position());
						let mob_name = mob.get_name();
                        	        	godot_print!("{mob_name} at {mob_path} being selected at {pos}");
					}
        	                	2 => {	
						self.chosen_mob = MobileKind::Cashier;
						let  mob: Gd<Node> = self.base().find_child("Cashier").expect("No cashier in tree!");
						let mob_path = mob.get_path();
						let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
						//Now set the position
						br_obj.set_position(mob_obj.get_position());
						let mob_name = mob.get_name();
                        	        	godot_print!("{mob_name} at {mob_path} being selected at {pos}");
					}
        	                	3 => {
						self.chosen_mob = MobileKind::WarehousePerson;
						let  mob: Gd<Node> = self.base().find_child("WarehousePerson").expect("No warehouseperson in tree!");
						let mob_path = mob.get_path();
						let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
						//Now set the position
						br_obj.set_position(mob_obj.get_position());
						let mob_name = mob.get_name();
						godot_print!("{mob_name} at {mob_path} being selected at {pos}");
					}
					_ => {
						//godot_print!("Customer or other unselectable");
					}
			
				}
			}//scope of print
		}//scope of match and print
	}

//	fn draw(&mut self) {
//		if self.draw_arc {
//			let col = Color::from_rgb(0.1, 1.0, 0.1);
//			let pos = self.base().get_position();
//			let arc_l = self.arc_length;
//			let draw_a = self.draw_arc;
//			if self.arc_length <= 0.0 {
//				self.arc_length = 1.57;
//				self.draw_arc = false;
//			}
//
//			let mut arc = self.base_mut();
//			arc.draw_arc_ex(pos, 300.0, 0.0, arc_l, 15, col).width(100.0).done();
//		}
//	}

	fn ready(&mut self) { 
		//add the Player items to the scene by adding them as children of the current node
		let sprite = self.base().find_child("ghost_boss_spr").expect("No ghost boss sprite in tree!");
		let spr: Gd<Sprite2D> = self.base_mut().get_node_as(&sprite.get_path());
		self.spr = spr;

		let timer = self.base().get_tree().expect("Not in a tree!")
				.create_timer(5.0).expect("No scene tree to speak of!");
		timer.signals().timeout().connect(Player::on_timer_done);
		
		self.signals()
			.boss_just_booped()
			.connect_self(Player::on_boss_just_booped);
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
		self.cook_timer.queue_free();
		self.spr.queue_free();
	}
}
