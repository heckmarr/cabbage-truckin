use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Package{
	pts: Vec<Vector2i>,
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
				godot_print!("row pts is {r}");
				match v {
					1 => {
						self.pts.push(Vector2i::new( 10, r ));
					}
					2 => {
						self.pts.push(Vector2i::new( 20, r ));
					}
					3 => {
						self.pts.push(Vector2i::new( 30, r ));
					}
					_ => {
						//pass
					}
				}
				
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
