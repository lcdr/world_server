use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		skill::SkillConstruction,
	},
};

use super::Component;

pub struct SkillComponent {

}

impl Component for SkillComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(SkillConstruction {
			skills_in_progress: None,
		})
	}
}
