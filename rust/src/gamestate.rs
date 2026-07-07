use godot::prelude::*;

use std::collections::HashMap;
use crate::mobiles::MobileKind;
use crate::mobiles::Mobiles;
use crate::select::BoundRect;

use godot::classes::Input;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameState {
	employees: HashMap<MobileKind, bool>,
	selected: HashMap<MobileKind, bool>,
	direction: i32,
	base: Base<Node2D>
	
}

use crate::player::Player;
#[godot_api]
impl GameState {
	#[signal]
	fn boss_transform();
	#[signal]
	fn boss_return_to_normal();
	#[signal]
	fn mob_die();
	fn bound_selection(&mut self, mob_kind: MobileKind, name: GString) {
		//Get the selection bounding rectangle and set it as visible
		let bounding_rect: Gd<Node> = self.base().find_child("BoundRect").expect("No bounding Rect in scene!");
		let br_path = bounding_rect.get_path();

		let mut br_obj: Gd<BoundRect> = bounding_rect.get_node_as(&br_path);
		//set before use
		let _pos = br_obj.get_position();
		br_obj.set_visible(true);
			
		//Get the next mobile in the pattern and its path
		let mob: Gd<Node> = self.base().find_child(&name).expect("{mob_name} is toast");
		let mob_path = mob.get_path();
		let mob_obj: Gd<Mobiles> = mob.get_node_as(&mob_path);
		//set position of selection
		br_obj.set_position(mob_obj.get_position());
		let name = mob.get_name();
		//get that position for pretty printing
		let pos = mob_obj.get_position();
//		godot_print!("{:?} at {:?} being selected at {:?}", name, mob_path, pos);
			
	}

	fn move_selection_left(&mut self) {
		for (employee_, selected) in self.selected.clone() {
			
			match employee_ {
				MobileKind::Cashier => {
					if selected {
						//if this is the one selected, move to the next
						let _ = &mut self.bound_selection(MobileKind::WarehousePerson,  "WarehousePerson".into());	
						self.selected.entry(MobileKind::Cashier).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::WarehousePerson).and_modify(|sel| *sel = true);

					}
				},
				MobileKind::WarehousePerson => {
					if selected {
						//if this is the one selected, move to the next
						let _ = &mut self.bound_selection(MobileKind::Chef, "Chef".into());
						self.selected.entry(MobileKind::WarehousePerson).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::Chef).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::Chef => {
					if selected {
						//if this is the one selected, move to the next					
						let _ = &mut self.bound_selection(MobileKind::Stocker, "Stocker".into());
						self.selected.entry(MobileKind::Chef).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::Stocker).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::Stocker => {
					if selected {
						//if this is the one selected, move to the next
						let _ = &mut self.bound_selection(MobileKind::Cashier, "Cashier".into());
						self.selected.entry(MobileKind::Stocker).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::Cashier).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::Customer => {
					if selected {
						//Should never reach this!
						break;
					}
				},
				MobileKind::Package => {
					if selected {
						//This either!
						break;
					}

				},
				
			}
		}

	}
	fn move_selection_right(&mut self) {
		for (employee_, selected) in self.selected.clone() {
			
			match employee_ {
				MobileKind::Stocker => {
					if selected {
						//if this is the one selected, move to the next
						let _ = &mut self.bound_selection(MobileKind::Chef,  "Chef".into());	
						self.selected.entry(MobileKind::Stocker).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::Chef).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::Chef => {
					if selected {
						//if this is the one selected, move to the next
						let _ = &mut self.bound_selection(MobileKind::WarehousePerson, "WarehousePerson".into());
						self.selected.entry(MobileKind::Chef).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::WarehousePerson).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::WarehousePerson=> {
					if selected {
						//if this is the one selected, move to the next					
						let _ = &mut self.bound_selection(MobileKind::Cashier, "Cashier".into());
						self.selected.entry(MobileKind::WarehousePerson).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::Cashier).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::Cashier => {
					if selected {
						//if this is the one selected, move to the next
						let _ = &mut self.bound_selection(MobileKind::Stocker, "Stocker".into());
						self.selected.entry(MobileKind::Cashier).and_modify(|sel| *sel = false);
						self.selected.entry(MobileKind::Stocker).and_modify(|sel| *sel = true);
					}
				},
				MobileKind::Customer => {
					if selected {
						//Should never reach this!
						break;
					}
				},
				MobileKind::Package => {
					if selected {
						//This either!
						break;
					}

				},
				
			}
		}

	}

	pub fn get_mobile(&mut self) -> Gd<Mobiles> {
		let mut mob_node = self.base().find_child("Package").expect("No package!");
		for (employee_, selected) in self.selected.clone() {
			
			match employee_ {
				MobileKind::Cashier => {
					if selected {
						//if this is the one selected, return it
						mob_node = self.base().find_child("Cashier").expect("No Cashier!");
					}
				},
				MobileKind::WarehousePerson => {
					if selected {
						//if this is the one selected, return it
						mob_node = self.base().find_child("WarehousePerson").expect("No WarehousePerson!");
					}
				},
				MobileKind::Chef => {
					if selected {
						//if this is the one selected, return it
						mob_node = self.base().find_child("Chef").expect("No Chef!");
					}
				},
				MobileKind::Stocker => {
					if selected {
						//if this is the one selected, return it
						mob_node = self.base().find_child("Stocker").expect("No Stocker!");
					}
				},
				MobileKind::Package => {
					break;
				},
				MobileKind::Customer => {
					break;
				}
			}
		}
		let mob_path = mob_node.get_path();
		let mob: Gd<Mobiles> = mob_node.get_node_as(&mob_path);
		return mob;
		
	}


	pub fn die(&mut self) {
		for (employee_, selected) in self.selected.clone() {
			
			match employee_ {
				MobileKind::Cashier => {
					if selected {
						//if this is the one selected, remove it
						self.selected.remove(&MobileKind::Cashier);						
					}
				},
				MobileKind::WarehousePerson => {
					if selected {
						//if this is the one selected, remove it
						self.selected.remove(&MobileKind::Cashier);						
					}
				},
				MobileKind::Chef => {
					if selected {
						//if this is the one selected, remove it					
						self.selected.remove(&MobileKind::Cashier);						
					}
				},
				MobileKind::Stocker => {
					if selected {
						//if this is the one selected, remove it
						self.selected.remove(&MobileKind::Cashier);						
					}
				},
				MobileKind::Customer => {
					if selected {
						//Should never reach this!
						break;
					}
				},
				MobileKind::Package => {
					if selected {
						//This either!
						break;
					}

				},
				
			}
		}
		
		
	}
}

#[godot_api]
impl INode2D for GameState {
	fn init(base: Base<Node2D>) -> Self {
		Self {
			
			direction: 0,
			//collections for selection and live checking
			selected: HashMap::new(),
			employees: HashMap::new(),
//			current: (MobileKind::Cashier, true),
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
		
		//start with the cashier
		//self.current = (MobileKind::Cashier, true);

		self.selected.insert(MobileKind::Cashier, true);
		self.selected.insert(MobileKind::Customer, false);
		self.selected.insert(MobileKind::Package, false);
		self.selected.insert(MobileKind::Chef, false);
		self.selected.insert(MobileKind::Stocker, false);
		self.selected.insert(MobileKind::WarehousePerson, false);
		
		//set the number of states you can change to
//		self.current = vec![0, 1, 2, 3];
		//Selected bounding rectangles
		//currently unused
		let _selected_mob = MobileKind::Cashier;
		self.bound_selection(MobileKind::Cashier, "Cashier".into());
		for (employee_type, _alive) in &self.employees {
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
		godot_print!("*****************GAMESTATE READY");
	}

	fn process(&mut self, _delta: f32) {
		//Draw the arc for the boss
		let player_node = self.base().find_child("Player").expect("Player is dead!");
		let player_path = player_node.get_path();
		let mut player: Gd<Player> = player_node.get_node_as(&player_path);
		//Deal with input
		let event = Input::singleton();

		//quit on esc
		if event.is_action_just_pressed("ui_cancel") {
			self.base().get_tree().quit();
		}
		//scare on select
		if event.is_action_just_pressed("ui_select") {
			//Also draw the arc
		}

		//transform the Player
		//todo draw the arc
		if event.is_action_just_pressed("ui_select") {
			player.signals().boss_just_transformed().emit();
			let mut al = player.bind_mut().get_arc_length();
			al -= 0.01745329;
			player.bind_mut().set_arc_length(al);
			player.bind_mut().set_draw_arc(true);
		}
		if event.is_action_just_released("ui_select") {
			player.signals().transform_the_boss().emit();
			player.bind_mut().set_draw_arc(false);
		}
		//Move the selection
		if event.is_action_just_pressed("ui_left") {
			self.direction = -1;
			self.move_selection_left();
		}

		if event.is_action_just_pressed("ui_right") {
			self.direction = 1;
			self.move_selection_right();
		}

		
		//regenerate the arc
		let mut al = player.bind_mut().get_arc_length();
		if al <= 3.0 {
			al += 0.08;
			player.bind_mut().set_arc_length(al);
		}
	}
}
