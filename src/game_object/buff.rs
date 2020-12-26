use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		buff::BuffConstruction,
	},
};

use super::Component;

pub struct BuffComponent {

}

impl Component for BuffComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(BuffConstruction {
			buffs: None,
			immunities: None,
		})
	}
}
