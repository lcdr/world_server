use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		player_forced_movement::{ForcedMovementInfo, PlayerForcedMovementConstruction},
	},
};

use super::Component;

pub struct PlayerForcedMovementComponent {

}

impl Component for PlayerForcedMovementComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(PlayerForcedMovementConstruction {
			forced_movement_info: Some(ForcedMovementInfo {
				player_on_rail: false,
				show_billboard: true,
			}),
		})
	}
}
