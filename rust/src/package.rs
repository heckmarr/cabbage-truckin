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
	
	fn append_next(&mut self, mut val: i32, tot: i32, mut num_in_row: i32) -> (i32, i32) {
		let choose = 4 - val;
		if choose <= 0 {
			return (0, num_in_row);
		}else {
			//this is the area you need to focus on, as it's calling Collect()
			//before it's added the value, causing it to skip one
			let v = randi_range(1, (choose - 1).into()) as i32;
			godot_print!("value chosen is {v}");
			if self.collect() >= tot || (self.collect() + v) > tot {
				return (0, num_in_row);
			}else {
				self.items.push(v);
				let r = ((tot / 4) - 1 ) * 10;
				let mut spr = Sprite2D::new_alloc();
				godot_print!("row pts is {r}");
				match v {
					1 => {
						num_in_row += 1;
						let mut pix = 0;
						if num_in_row == 1 {
							//pass
						}else {
							pix = num_in_row * 10;
						}
						self.pts.push(Vector2i::new( pix, r ));
						spr.set_position(Vector2::new( pix as f32, r as f32));
						let tex = load("res://sprites/one-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
					}
					2 => {
						num_in_row += 2;
						let mut pix = 0;
						if num_in_row == 2 {
							//pass
						} else {
							pix = num_in_row * 10;
						}
						self.pts.push(Vector2i::new( pix, r ));
						spr.set_position(Vector2::new(pix as f32, r as f32));
						let tex = load("res://sprites/two-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
					}
					3 => {
						num_in_row += 3;
						let mut pix = 0;
						if num_in_row == 3 {
							//pass
						}else {
							pix = num_in_row * 10;
						}
						self.pts.push(Vector2i::new( pix,  r));
						spr.set_position(Vector2::new( pix as f32, r as f32));
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
		return (val, num_in_row);
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
		let mut num_in_row = 0;
		for r in 1..(self.rows + 1) {
//			godot_print!("num of row is {r}");
			while self.collect() < (4 * r) {
				let v = val;
				let tot = 4 * r;
				(val, num_in_row) = self.append_next(v, tot, num_in_row);
//				godot_print!("collect() is {0}", self.collect());
			}
			val = 0;
			num_in_row = 0;
		}
		godot_print!("collect() is {0}", self.collect());

	}

}
