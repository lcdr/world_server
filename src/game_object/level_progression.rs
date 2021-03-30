use lu_packets::{
	raknet::client::replica::level_progression::{LevelProgressionConstruction, LevelProgressionProtocol, LevelProgressionSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct LevelProgressionComponent {

}

impl InternalComponent for LevelProgressionComponent {
	type ComponentProtocol = LevelProgressionProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> LevelProgressionConstruction {
		LevelProgressionConstruction {
			current_level: None,
		}
	}

	fn make_serialization(&self) -> LevelProgressionSerialization {
		LevelProgressionSerialization {
			current_level: None,
		}
	}
}
