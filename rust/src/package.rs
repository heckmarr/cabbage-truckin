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
	
	fn append_next(&mut self, tot: i32, mut num_in_row: i32) -> (i32) {
		let choose = 4 - num_in_row;
		if choose <= 0 {
			return (num_in_row);
		}else {
			//this is the area you need to focus on, as it's calling Collect()
			//before it's added the value, causing it to skip one
			if choose == 1 {
				godot_print!("Accepted 1");
				self.items.push(1);
				let r = ((tot / 4) - 1 ) * 10;
				let mut spr = Sprite2D::new_alloc();
				godot_print!("row pts is {r}");
				let preceding = num_in_row;
						godot_print!("preceding value is {preceding}");

				num_in_row += 1;
				let mut pix = 0;
				if preceding == 0 {
					//pass
				}else {
//					pix = 10;
					pix = (preceding * 10);
//					pix = preceding * 10;
				}
				let tex = load("res://sprites/one-block.png") as Gd<Texture2D>;
				spr.set_texture(&tex);
				self.pts.push(Vector2i::new( pix, r ));
				spr.set_global_position(Vector2::new( pix as f32, r as f32));
				godot_print!("pix is {0}", pix as f32);
				self.base_mut().call_deferred("add_sibling", &[spr.to_variant()]);
				return (num_in_row);
			}
			let v = randi_range(1, 3) as i32;
			godot_print!("value chosen is {v}");
			if (num_in_row + v) > 4 {
				godot_print!("Rejected {v}");
				return (num_in_row);
			}else {
				godot_print!("Accepted {v}");
				self.items.push(v);
				let r = ((tot / 4) - 1 ) * 10;
				let mut spr = Sprite2D::new_alloc();
				godot_print!("row pts is {r}");
				match v {
					1 => {
						let preceding = num_in_row;
						godot_print!("preceding value is {preceding}");

						num_in_row += 1;
						let mut pix = 0;
						if preceding == 0 {
							//pass
						}else {
//							pix = 10;
							pix = (preceding * 10);
						}
						self.pts.push(Vector2i::new( pix, r ));
						let tex = load("res://sprites/one-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
						spr.set_global_position(Vector2::new( pix as f32, r as f32));
						self.base_mut().call_deferred("add_sibling", &[spr.to_variant()]);
						godot_print!("pix is {0}", pix as f32);
						return num_in_row;

					}
					2 => {
						let preceding = num_in_row;
						godot_print!("preceding value is {preceding}");

						num_in_row += 2;
						let mut pix = 0;
						if preceding == 0 {
							//pass
						} else {
//							pix = 20;
							pix = (preceding * 10);
//							pix = preceding * 10;
						}
						self.pts.push(Vector2i::new( pix, r ));
						let tex = load("res://sprites/two-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
						spr.set_global_position(Vector2::new(pix as f32, r as f32));
						self.base_mut().call_deferred("add_sibling", &[spr.to_variant()]);
						godot_print!("pix is {0}", pix as f32);
						return num_in_row;
					}
					3 => {
						let preceding = num_in_row;
						godot_print!("preceding value is {preceding}");
						num_in_row += 3;
						let mut pix = 0;
						if preceding == 0 {
							//pass
						}else {
//							pix = 30;
							pix = (preceding * 10);
//							pix = preceding * 10;
						}
						self.pts.push(Vector2i::new( pix,  r));
						let tex = load("res://sprites/three-block.png") as Gd<Texture2D>;
						spr.set_texture(&tex);
						spr.set_global_position(Vector2::new( pix as f32, r as f32));
						self.base_mut().call_deferred("add_sibling", &[spr.to_variant()]);
						godot_print!("pix is {0}", pix as f32);
						return num_in_row;
					}
					_ => {
						//pass
					}
				}
//				self.sprites.push(spr.clone());
				

			}
		}
		return (-1);
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
		let mut num_in_row = 0;
		for r in 1..(self.rows + 1) {
//			godot_print!("num of row is {r}");
			while num_in_row < 4 {
				let tot = 4 * r;
				let n = num_in_row;
				(num_in_row) = self.append_next(tot, num_in_row);
				if num_in_row == -1 {
					num_in_row = n;
				}
//				godot_print!("collect() is {0}", self.collect());
			}
			godot_print!("ROW {r} FINISHED");
			num_in_row = 0;
		}
		godot_print!("collect() is {0}", self.collect());

	}

}
