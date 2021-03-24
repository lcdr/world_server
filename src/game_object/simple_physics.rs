use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		simple_physics::{PositionRotationInfo, SimplePhysicsConstruction},
	},
	world::{Quaternion, Vector3},
};

use super::Component;

pub struct SimplePhysicsComponent {

}

impl Component for SimplePhysicsComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(SimplePhysicsConstruction {
			climbing_property: None,
			velocity_info: None,
			motion_type: None,
			position_rotation_info: Some(PositionRotationInfo {
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
			}),
		})
	}
}
