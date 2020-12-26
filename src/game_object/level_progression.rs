use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		level_progression::LevelProgressionConstruction,
	},
};

use super::Component;

pub struct LevelProgressionComponent {

}

impl Component for LevelProgressionComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(LevelProgressionConstruction {
			current_level: Some(1),
		})
	}
}
