use lu_packets::{
	raknet::client::replica::player_forced_movement::{PlayerForcedMovementConstruction, PlayerForcedMovementProtocol, PlayerForcedMovementSerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct PlayerForcedMovementComponent {

}

impl InternalComponent for PlayerForcedMovementComponent {
	type ComponentProtocol = PlayerForcedMovementProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> PlayerForcedMovementConstruction {
		PlayerForcedMovementConstruction {
			forced_movement_info: None,
		}
	}

	fn make_serialization(&self) -> PlayerForcedMovementSerialization {
		PlayerForcedMovementSerialization {
			forced_movement_info: None,
		}
	}
}
