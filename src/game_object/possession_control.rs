use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		possession_control::{PossessionControlConstruction, PossessionInfo, PossessionType},
	},
	world::LuNameValue,
};

use super::Component;

pub struct PossessionControlComponent {

}

impl Component for PossessionControlComponent {
	fn new(_config: &LuNameValue) -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(PossessionControlConstruction {
			possession_info: Some(PossessionInfo {
				possessed_id: None,
				possession_type: PossessionType::NoPossession,
			}),
		})
	}
}
