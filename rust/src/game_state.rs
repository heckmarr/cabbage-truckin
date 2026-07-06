use godot::prelude::*;

use std::collections::HashMap;
use crate::mobiles::MobileKind;

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
			match employee_ {
				MobileKind::Cashier => {
					if *selected {
						break;
					}
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
	}

	fn process(&mut self, _delta: f32) {

	}
}
