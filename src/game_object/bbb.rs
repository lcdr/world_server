use lu_packets::{
	raknet::client::replica::bbb::{BbbConstruction, BbbProtocol, BbbSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct BbbComponent {

}

impl InternalComponent for BbbComponent {
	type ComponentProtocol = BbbProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> BbbConstruction {
		BbbConstruction {
			metadata_source_item: None,
		}
	}

	fn make_serialization(&self) -> BbbSerialization {
		BbbSerialization {
			metadata_source_item: None,
		}
	}
}
