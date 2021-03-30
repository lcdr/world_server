use lu_packets::{
	raknet::client::replica::possession_control::{PossessionControlConstruction, PossessionControlProtocol, PossessionControlSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct PossessionControlComponent {

}

impl InternalComponent for PossessionControlComponent {
	type ComponentProtocol = PossessionControlProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> PossessionControlConstruction {
		PossessionControlConstruction {
			possession_info: None,
		}
	}

	fn make_serialization(&self) -> PossessionControlSerialization {
		PossessionControlSerialization {
			possession_info: None,
		}
	}
}
