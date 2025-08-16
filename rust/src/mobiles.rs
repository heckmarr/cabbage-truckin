use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;
use godot::classes::Node2D;
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

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Mobiles {
	timer: Gd<Timer>,
	#[export]
	mob: MobileKind,
	base: Base<Sprite2D>,
}


#[derive(GodotClass)]
#[class(base=Node2D)]
struct Packaging {
	hitpoints: i32,
	base: Base<Node2D>,
}
use godot::global::randi_range;

#[godot_api]
impl ISprite2D for Mobiles {
	fn init(base: Base<Sprite2D>) -> Self {
		godot_print!("Mobile ready");
		Self {
			timer: NewAlloc::new_alloc(),
			mob: MobileKind::Chef,
			base,
		}
	}
}

impl Drop for Mobiles {
	fn drop(&mut self) {
		godot_print!("Dropping {0}", self.timer);
		self.timer.queue_free();
	}
}
#[godot_api]
impl Mobiles {
	#[signal]
	fn balete();
}

#[godot_api]
impl Packaging {
	#[signal]
	fn packaging_random_damage_taken(amount: i32);
	#[signal]
	fn packaging_damage_taken(amount: i32);
	#[func]
	fn packaging_damage_emit(&mut self, amount: i32) {
		self.signals().packaging_damage_taken().emit(amount);
	}
	#[func]
	fn packaging_random_damage_emit(&mut self) {
		//The random number range is 100-500
		let rand_num = randi_range(100, 500) as i32;
		self.signals().packaging_random_damage_taken().emit(rand_num);
	}
	fn on_packaging_damage_taken(&mut self, amount: i32) {
		self.hitpoints -= amount;
		let mut hp = self.hitpoints;
		//stop at zero! He's dead already!
		if hp < 0 {
			hp = 0
		}
		godot_print!("packaging taking {amount} damage of {hp} total");
	}

}

#[godot_api]
impl INode2D for Packaging{
	fn init(base: Base<Node2D>) -> Self {
		godot_print!("Initializing damageable packaging");
		Self {
			hitpoints: 100,
			base,
		}
	}
	fn ready(&mut self)  {
		godot_print!("Connecting signals for packaging");
		self.signals()
			.packaging_damage_taken()
			.connect_self(Self::on_packaging_damage_taken);
		self.signals()
			.packaging_random_damage_taken()
			.connect_self(Self::on_packaging_damage_taken);
	}
}
