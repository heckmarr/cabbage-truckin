use godot::prelude::*;

use std::collections::HashMap;
use crate::mobiles::MobileKind;
use crate::mobiles::Mobiles;
use crate::select::BoundRect;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameState {
	employees: HashMap<MobileKind, bool>,
	selected: HashMap<MobileKind, bool>,
	base: Base<Node2D>
	
}

#[godot_api]
impl GameState {
	#[signal]
	fn mob_die();

	fn die(&mut self) {
		let cash = self.employees.entry(MobileKind::Cashier);
		
		
	}
}

#[godot_api]
impl INode2D for GameState {
	fn init(base: Base<Node2D>) -> Self {
		Self {
			

			//collections for selection and live checking
			selected: HashMap::new(),
			employees: HashMap::new(),
			base,
		}
	}

	fn ready(&mut self) {
		self.employees.insert(MobileKind::Cashier, true);
		self.employees.insert(MobileKind::Customer, true);
		self.employees.insert(MobileKind::Package, true);
		self.employees.insert(MobileKind::Chef, true);
		self.employees.insert(MobileKind::Stocker, true);
		self.employees.insert(MobileKind::WarehousePerson, true);
		
		self.selected.insert(MobileKind::Cashier, true);
		self.selected.insert(MobileKind::Customer, false);
		self.selected.insert(MobileKind::Package, false);
		self.selected.insert(MobileKind::Chef, false);
		self.selected.insert(MobileKind::Stocker, false);
		self.selected.insert(MobileKind::WarehousePerson, false);

		//Selected bounding rectangles
		let selected_mob = MobileKind::Cashier;

		for (employee_type, alive) in &self.employees {
			match employee_type {
				MobileKind::Cashier => {
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Package => {
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::WarehousePerson => {
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Chef => {
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Stocker => {
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Customer => {
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
			}
		}
		for (employee_, selected) in &self.selected {
			
			//Get the selection bounding rectangle and set it as visible
			let mut bounding_rect: Gd<Node> = self.base().find_child("BoundRect").expect("No bounding Rect in scene!");
			let br_path = bounding_rect.get_path();

			let mut br_obj: Gd<BoundRect> = bounding_rect.get_node_as(&br_path);
			let pos = br_obj.get_position();
			br_obj.set_visible(true);
			match employee_ {
				MobileKind::Cashier => {
					if *selected {
						//Get the mobile and it's path
						let mob: Gd<Node> = self.base().find_child("Cashier").expect("Cashier is toast");
						let mob_path = mob.get_path();
						let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
						//set position of selection
						br_obj.set_position(mob_obj.get_position());
						let mob_name = mob.get_name();
						//get that position for pretty printing
						let pos = mob_obj.get_position();
						godot_print!("{:?} at {:?} being selected at {:?}", mob_name, mob_path, pos);
						
					}
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Package => {
					if *selected {
						break;
					}

					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::WarehousePerson => {
					if *selected {
						break;
					}
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Chef => {
					if *selected {
						break;
					}
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Stocker => {
					if *selected {
						break;
					}
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
				MobileKind::Customer => {
					if *selected {
						break;
					}
					//Todo Fill in with all other mob types

					//let cashier = self.base().find_child("Cashier").expect("Cashier is already dead!");
					//cashier.signals().mob_die().connect_self(Self::die);
				},
			}
		}
	}

	fn process(&mut self, _delta: f32) {

	}
}
