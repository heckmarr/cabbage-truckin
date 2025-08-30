use godot::prelude::*;
use godot::obj::Gd;
use godot::classes::Node2D;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BoundRect {
	top_left: Vector2,
	top_right: Vector2,
	bottom_left: Vector2,
	bottom_right: Vector2,
	base: Base<Node2D>
}



use godot::classes::INode2D;

#[godot_api]
impl INode2D for BoundRect {
	fn init(base: Base<Node2D>) -> Self {
		godot_print!("Initializing target bounding box"); //Prints to the godot console

		Self {
			top_left: Vector2::new(-75.0, 0.0),
			top_right: Vector2::new(75.0, 0.0),
			bottom_left: Vector2::new(-75.0, 75.0),
			bottom_right: Vector2::new(75.0, 75.0),
			base,
		}
	}
	fn ready(&mut self) {
		let tlpos = self.top_right;
		godot_print!("{tlpos} top right position in ready");
		//set all the positions
		let mut tl: Gd<Node2D> = self.base_mut().get_node_as("TopLeft");
		tl.set_position(self.top_left);
		let mut tr: Gd<Node2D> = self.base_mut().get_node_as("TopRight");
		tr.set_position(self.top_right);
		let mut bl: Gd<Node2D> = self.base_mut().get_node_as("BottomLeft");
		bl.set_position(self.bottom_left);
		let mut br: Gd<Node2D> = self.base_mut().get_node_as("BottomRight");
		br.set_position(self.bottom_right);
	}

}

