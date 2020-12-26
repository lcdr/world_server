use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		bbb::BbbConstruction,
	},
};

use super::Component;

pub struct BbbComponent {

}

impl Component for BbbComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(BbbConstruction {
			metadata_source_item: Some(0),
		})
	}
}
