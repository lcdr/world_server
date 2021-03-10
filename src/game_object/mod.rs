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
mod skill;

use std::io::Result as Res;

use lu_packets::{
	lu,
	common::{LuVarWString, ObjId},
	raknet::client::replica::{ComponentConstruction, ParentChildInfo, ReplicaConstruction},
	world::gm::client::{SubjectGameMessage as ClientSGM, GameMessage as ClientGM},
	world::gm::server::GameMessage as ServerGM,
};

use crate::listeners::Context;
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
use self::skill::SkillComponent;

trait Component {
	fn new() -> Box<dyn Component> where Self: Sized;
	fn make_construction(&self) -> Box<dyn ComponentConstruction>;
	fn write_xml(&self, _writer: &mut String) -> std::fmt::Result {
		Ok(())
	}
}

pub struct GameObject {
	network_id: u16,
	object_id: ObjId,
	name: LuVarWString<u8>,
	components: Vec<Box<dyn Component>>,
}

impl GameObject {
	pub fn new(network_id: u16, object_id: ObjId) -> Self {
		Self {
			network_id,
			object_id,
			name: lu!(&format!("{}", object_id)[..]),
			components: vec![
				ControllablePhysicsComponent::new(),
				BuffComponent::new(),
				DestroyableComponent::new(),
				PossessionControlComponent::new(),
				LevelProgressionComponent::new(),
				PlayerForcedMovementComponent::new(),
				CharacterComponent::new(),
				InventoryComponent::new(),
				SkillComponent::new(),
				FxComponent::new(),
				BbbComponent::new(),
			],
		}
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
			lot: 1,
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

	pub fn make_sgm(&self, message: ClientGM) -> ClientSGM {
		ClientSGM {
			subject_id: self.object_id,
			message,
		}
	}

	pub fn on_game_message(&mut self, msg: &ServerGM, _ctx: &mut Context) -> Res<()> {
		dbg!(msg);
		Ok(())
	}
}