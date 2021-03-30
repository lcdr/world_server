use lu_packets::{
	raknet::client::replica::fx::{FxConstruction, FxProtocol, FxSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct FxComponent {

}

impl InternalComponent for FxComponent {
	type ComponentProtocol = FxProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> FxConstruction {
		FxConstruction {
			active_effects: vec![].into(),
		}
	}

	fn make_serialization(&self) -> FxSerialization {
		FxSerialization {}
	}
}
