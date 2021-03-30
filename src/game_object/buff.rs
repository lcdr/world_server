use lu_packets::{
	raknet::client::replica::buff::{BuffConstruction, BuffProtocol, BuffSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct BuffComponent {

}

impl InternalComponent for BuffComponent {
	type ComponentProtocol = BuffProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> BuffConstruction {
		BuffConstruction {
			buffs: None,
			immunities: None,
		}
	}

	fn make_serialization(&self) -> BuffSerialization {
		BuffSerialization {}
	}
}
