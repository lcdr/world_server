use lu_packets::{
	raknet::client::replica::script::{ScriptConstruction, ScriptProtocol, ScriptSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct ScriptComponent {

}

impl InternalComponent for ScriptComponent {
	type ComponentProtocol = ScriptProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> ScriptConstruction {
		ScriptConstruction {
			network_vars: None,
		}
	}

	fn make_serialization(&self) -> ScriptSerialization {
		ScriptSerialization {}
	}
}
