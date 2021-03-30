use lu_packets::{
	raknet::client::replica::skill::{SkillConstruction, SkillProtocol, SkillSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct SkillComponent {

}

impl InternalComponent for SkillComponent {
	type ComponentProtocol = SkillProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> SkillConstruction {
		SkillConstruction {
			skills_in_progress: None,
		}
	}

	fn make_serialization(&self) -> SkillSerialization {
		SkillSerialization {}
	}
}
