use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		skill::SkillConstruction,
	},
	world::LuNameValue,
};

use super::Component;

pub struct SkillComponent {

}

impl Component for SkillComponent {
	fn new(_config: &LuNameValue) -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(SkillConstruction {
			skills_in_progress: None,
		})
	}
}
