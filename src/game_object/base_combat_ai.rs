use lu_packets::{
	raknet::client::replica::base_combat_ai::{BaseCombatAiConstruction, BaseCombatAiProtocol, BaseCombatAiSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct BaseCombatAiComponent {

}

impl InternalComponent for BaseCombatAiComponent {
	type ComponentProtocol = BaseCombatAiProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> BaseCombatAiConstruction {
		BaseCombatAiConstruction {
			combat_ai_info: None,
		}
	}

	fn make_serialization(&self) -> BaseCombatAiSerialization {
		BaseCombatAiSerialization {
			combat_ai_info: None,
		}
	}
}
