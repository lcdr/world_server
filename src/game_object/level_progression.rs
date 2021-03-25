use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		level_progression::LevelProgressionConstruction,
	},
	world::LuNameValue,
};

use super::Component;

pub struct LevelProgressionComponent {

}

impl Component for LevelProgressionComponent {
	fn new(_config: &LuNameValue) -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(LevelProgressionConstruction {
			current_level: None,
		})
	}
}
