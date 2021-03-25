use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		player_forced_movement::{PlayerForcedMovementConstruction},
	},
	world::LuNameValue,
};

use super::Component;

pub struct PlayerForcedMovementComponent {

}

impl Component for PlayerForcedMovementComponent {
	fn new(_config: &LuNameValue) -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(PlayerForcedMovementConstruction {
			forced_movement_info: None,
		})
	}
}
