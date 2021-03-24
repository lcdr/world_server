use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		controllable_physics::{ControllablePhysicsConstruction, FrameStats},
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
			stun_immunity_info: None,
			cheat_info: None,
			unknown_1: None,
			unknown_2: None,
			frame_stats: Some(FrameStats {
				position: Vector3 {
					x: 156.0,
					y: 380.0,
					z: -187.0,
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
