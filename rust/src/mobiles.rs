use godot::prelude::*;
use godot::classes::AnimatedSprite2D;
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

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Mobiles {
	hitpoints: i32,
	timer: Gd<Timer>,
	#[export]
	mob: MobileKind,
	anim: Gd<AnimatedSprite2D>,
	base: Base<Node2D>,
}


use godot::global::randi_range;

#[godot_api]
impl INode2D for Mobiles {
	fn init(base: Base<Node2D>) -> Self {
		godot_print!("Mobile ready");
		Self {
			hitpoints: 100,
			timer: Timer::new_alloc(),
			mob: MobileKind::Customer,
			anim: AnimatedSprite2D::new_alloc(),
			base,
		}
	}
	fn ready(&mut self)  {
		godot_print!("Connecting signals for mobiles");
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
					godot_print!("Chef position moving to x: {pos}");
			}
			MobileKind::Customer => {godot_print!("Customer!")}
			MobileKind::Stocker => {godot_print!("Stocker!")}
			MobileKind::Cashier => {godot_print!("Cashier!")}
			MobileKind::Package => {godot_print!("Package!")}
			MobileKind::WarehousePerson => {godot_print!("WarehousePerson!")}
		}
	}
}

impl Drop for Mobiles {
	fn drop(&mut self) {
		godot_print!("Dropping {0}", self.timer);
		self.timer.queue_free();
		godot_print!("Dropping {0}", self.anim);
		self.anim.queue_free();
	}
}
#[godot_api]
impl Mobiles {
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

}
