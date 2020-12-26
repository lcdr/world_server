use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		controllable_physics::{ControllablePhysicsConstruction, FrameStats, StunImmunityInfo, Unknown1, Unknown2},
	},
	world::{Quaternion, Vector3},
};

use super::Component;

pub struct ControllablePhysicsComponent {

}

impl Component for ControllablePhysicsComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(ControllablePhysicsConstruction {
			jetpack_info: None,
			stun_immunity_info: Some(StunImmunityInfo {
				immune_to_stun_move: 0,
				immune_to_stun_jump: 0,
				immune_to_stun_turn: 0,
				immune_to_stun_attack: 0,
				immune_to_stun_use_item: 0,
				immune_to_stun_equip: 0,
				immune_to_stun_interact: 0,
			}),
			cheat_info: None,
			unknown_1: Some(Unknown1 {
				unknown_1: 0.0,
				unknown_2: false,
			}),
			unknown_2: Some(Unknown2 {
				unknown_1: None,
			}),
			frame_stats: Some(FrameStats {
				position: Vector3 {
					x: -627.1862182617188,
					y: 613.3262329101562,
					z: -17.223167419433594,
				},
				rotation: Quaternion {
					x: 0.0,
					y: 0.7334349751472473,
					z: 0.0,
					w: 0.6797596216201782,
				},
				is_on_ground: true,
				is_on_rail: false,
				linear_velocity: None,
				angular_velocity: None,
				local_space_info: None,
			}),
		})
	}
}
