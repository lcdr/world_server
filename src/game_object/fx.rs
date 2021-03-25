use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		fx::FxConstruction,
	},
	world::LuNameValue,
};

use super::Component;

pub struct FxComponent {

}

impl Component for FxComponent {
	fn new(_config: &LuNameValue) -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(FxConstruction {
			active_effects: vec![].into(),
		})
	}
}
