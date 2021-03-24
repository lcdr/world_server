use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		script::ScriptConstruction,
	},
};

use super::Component;

pub struct ScriptComponent {

}

impl Component for ScriptComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(ScriptConstruction {
			network_vars: None,
		})
	}
}