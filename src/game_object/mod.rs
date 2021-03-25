mod bbb;
mod buff;
mod character;
mod controllable_physics;
mod destroyable;
mod fx;
mod inventory;
mod level_progression;
mod player_forced_movement;
mod possession_control;
mod script;
mod simple_physics;
mod skill;

use std::io::Result as Res;

use rusqlite::{Connection as RusqliteConnection, params};

use lu_packets::{
	lu,
	common::{LuVarWString, ObjId},
	raknet::client::replica::{ComponentConstruction, ParentChildInfo, ReplicaConstruction},
	world::{Lot, LuNameValue},
	world::gm::client::{SubjectGameMessage as ClientSGM, GameMessage as ClientGM},
	world::gm::server::GameMessage as ServerGM,
};

use crate::listeners::Context;
use crate::listeners::MsgCallback;
use crate::services::GameObjectService;
use self::bbb::BbbComponent;
use self::buff::BuffComponent;
use self::character::CharacterComponent;
use self::controllable_physics::ControllablePhysicsComponent;
use self::destroyable::DestroyableComponent;
use self::fx::FxComponent;
use self::inventory::InventoryComponent;
use self::level_progression::LevelProgressionComponent;
use self::player_forced_movement::PlayerForcedMovementComponent;
use self::possession_control::PossessionControlComponent;
use self::script::ScriptComponent;
use self::simple_physics::SimplePhysicsComponent;
use self::skill::SkillComponent;

trait Component {
	fn new(config: &LuNameValue) -> Box<dyn Component> where Self: Sized;
	fn make_construction(&self) -> Box<dyn ComponentConstruction>;
	fn write_xml(&self, _writer: &mut String) -> std::fmt::Result {
		Ok(())
	}
	fn on_game_message(&mut self, _msg: &ServerGM, _game_object: &mut GameObject, _server: &mut MsgCallback, _ctx: &mut Context) {}
	fn run_service(&self, _service: &mut GameObjectService) {}
}

pub struct GameObject {
	network_id: u16,
	object_id: ObjId,
	lot: Lot,
	name: LuVarWString<u8>,
	components: Vec<Box<dyn Component>>,
}

const COMP_ORDER: [u32; 35] = [108, 61, 1, 30, 20, 3, 40, 98, 7, 110, 109, 106, 4, 26, 17, 5, 9, 60, 11, 48, 25, 16, 100, 102, 19, 39, 23, 75, 42, 6, 49, 2, 44, 71, 107];

impl GameObject {
	pub fn new(network_id: u16, object_id: ObjId, lot: Lot, config: &LuNameValue, cdclient: &RusqliteConnection) -> Self {

		let mut stmt = cdclient.prepare("select component_type from componentsregistry where id = ?").unwrap();
		let mut comps: Vec<u32> = stmt.query_map(params![lot], |row| row.get(0)).unwrap().map(|x| x.unwrap()).collect();
		dbg!(&comps);

		comps.sort_by_key(|x| COMP_ORDER.iter().position(|y| y == x).unwrap_or(usize::MAX));
		comps.dedup();

		let mut final_comps = vec![];
		Self::apply_component_overrides(&comps, &mut final_comps);

		let components = Self::create_components(&final_comps, config);

		Self {
			network_id,
			object_id,
			lot,
			name: lu!(&format!("{}", object_id)[..]),
			components,
		}
	}

	fn apply_component_overrides(comps: &Vec<u32>, final_comps: &mut Vec<u32>) {
		for comp in comps {
			// special case: utter bodge
			match comp {
				2  => { final_comps.push(44); }
				4  => { final_comps.push(110); final_comps.push(109); final_comps.push(106); }
				7  => { final_comps.push(98); }
				23 | 48 => {
					if !final_comps.contains(&7) {
						final_comps.push(7);
					}
				}
				_ => {},
			}
			final_comps.push(*comp);
		}
		// special case: utter bodge
		if final_comps.contains(&26) {
			final_comps.remove(final_comps.iter().position(|&x| x == 11).unwrap());
			final_comps.remove(final_comps.iter().position(|&x| x == 42).unwrap());
		}
	}

	fn create_components(comps: &Vec<u32>, config: &LuNameValue) -> Vec<Box<dyn Component>> {
		let mut components = vec![];

		for comp in comps {
			if let 2 | 12 | 24 | 27 | 31 | 35 | 36 | 43 | 45 | 55 | 56 | 57 | 64 | 65 | 67 | 68 | 73 | 74 | 78 | 95 | 104 | 113 | 114 = comp {
			} else {
				components.push(match comp {
					1  =>  ControllablePhysicsComponent::new,
					3  =>  SimplePhysicsComponent::new,
					4  =>  CharacterComponent::new,
					5  =>  ScriptComponent::new,
					7  =>  DestroyableComponent::new,
					9  =>  SkillComponent::new,
					17 =>  InventoryComponent::new,
					44 =>  FxComponent::new,
					98 =>  BuffComponent::new,
					106 => PlayerForcedMovementComponent::new,
					107 => BbbComponent::new,
					109 => LevelProgressionComponent::new,
					110 => PossessionControlComponent::new,
					x => panic!("{}", x),
				}(config));
			}
		}
		components
	}

	pub fn object_id(&self) -> ObjId {
		return self.object_id;
	}

	pub fn name(&self) -> &LuVarWString<u8> {
		return &self.name;
	}

	pub fn make_construction(&self) -> ReplicaConstruction {
		let mut comp_constructions = vec![];

		for comp in &self.components {
			comp_constructions.push(comp.make_construction());
		}

		ReplicaConstruction {
			network_id: self.network_id,
			object_id: self.object_id,
			lot: self.lot,
			name: lu!("GruntMonkey"),
			time_since_created_on_server: 0,
			config: None,
			is_trigger: false,
			spawner_id: None,
			spawner_node_id: None,
			scale: None,
			world_state: None,
			gm_level: None,
			parent_child_info: Some(ParentChildInfo {
				parent_info: None,
				child_info: None,
			}),
			components: comp_constructions,
		}
	}

	pub fn write_xml(&self, writer: &mut String) -> std::fmt::Result {
		use std::fmt::Write;
		write!(writer, "<obj v=\"1\">")?;
		for comp in &self.components {
			comp.write_xml(writer)?;
		}
		write!(writer, "</obj>")?;
		Ok(())
	}

	pub fn make_sgm<T: Into<ClientGM>>(&self, message: T) -> ClientSGM {
		ClientSGM {
			subject_id: self.object_id,
			message: message.into(),
		}
	}

	pub fn on_game_message(&mut self, msg: &ServerGM, server: &mut MsgCallback, ctx: &mut Context) -> Res<()> {
		dbg!(msg);

		for i in 0..self.components.len() {
			let mut comp = self.components.swap_remove(i);
			comp.on_game_message(msg, self, server, ctx);
			self.components.push(comp);
			if i > 0 {
				self.components.swap(i, i-1);
			}
		}
		Ok(())
	}

	pub fn run_service<'a, S: Into<GameObjectService<'a>>>(&self, service: S) {
		let mut go_service = service.into();
		for comp in &self.components {
			comp.run_service(&mut go_service);
		}
	}
}