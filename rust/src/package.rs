use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;
use godot::classes::Sprite2D;
use godot::classes::Texture2D;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Package{
	pts: Vec<Vector2i>,
	sprites: Vec<Gd<Sprite2D>>,
	items: Vec<i32>,
	rows: i32,
	base: Base<Node2D>
}

use godot::global::randi_range;
#[godot_api]
impl Package {
	
	fn append_next(&mut self, mut val: i32, tot: i32) -> i32 {
		let choose = 4 - val;
		if choose <= 0 {
			return 0;
		}else {
			let v = randi_range(1, (choose - 1).into()) as i32;
//			godot_print!("value chosen is {v}");
			if self.collect() >= tot || (self.collect() + v) > tot {
				return 0;
			}else {
				self.items.push(v);
				let r = ((tot / 4) - 1 ) * 10;
				let mut spr = Sprite2D::new_alloc();
				godot_print!("row pts is {r}");
				match v {
					1 => {
						self.pts.push(Vector2i::new( 10, r ));
						spr.set_position(Vector2::new( 10.0, r as f32));
						let tex = load("res://sprites/one-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
					}
					2 => {
						self.pts.push(Vector2i::new( 20, r ));
						spr.set_position(Vector2::new(20.0, r as f32));
						let tex = load("res://sprites/two-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
					}
					3 => {
						self.pts.push(Vector2i::new( 30, r));
						spr.set_position(Vector2::new( 30.0, r as f32));
						let tex = load("res://sprites/three-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
					}
					_ => {
						//pass
					}
				}
				self.sprites.push(spr.clone());
				
				self.base_mut().call_deferred("add_sibling", &[spr.to_variant()]);

				val = v
			}
		}
		return val;
	}
	fn collect(&mut self) -> i32 {
		let mut tot = 0;
		for i in self.items.iter() {
			tot += i;
		}
		godot_print!("total items in items {tot}");
		return tot
	}
}

#[godot_api]
impl INode2D for Package {
	fn init(base: Base<Node2D>) -> Self {
		Self {
			items: Vec::new(),
			rows: 4,
			pts: Vec::<Vector2i>::new(),
			sprites: Vec::<Gd<Sprite2D>>::new(),
			base,
		}
	}

	fn ready(&mut self) {
		let mut val = 0;
		for r in 1..(self.rows + 1) {
//			godot_print!("num of row is {r}");
			while self.collect() < (4 * r) {
				let v = val;
				let tot = 4 * r;
				val = self.append_next(v, tot);
//				godot_print!("collect() is {0}", self.collect());
			}
			val = 0
		}
		godot_print!("collect() is {0}", self.collect());

	}

}
