use lu_packets::{
	lu,
	raknet::client::replica::simple_physics::{PositionRotationInfo, SimplePhysicsConstruction, SimplePhysicsProtocol, SimplePhysicsSerialization},
	world::{LuNameValue, LnvValue, Quaternion, Vector3},
};

use super::InternalComponent;

pub struct SimplePhysicsComponent {
	position: Vector3,
}

impl InternalComponent for SimplePhysicsComponent {
	type ComponentProtocol = SimplePhysicsProtocol;

	fn new(config: &LuNameValue) -> Self {
		let x = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_x")) { *x } else { 156.0 };
		let y = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_y")) { *x } else { 380.0 };
		let z = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_z")) { *x } else { -187.0 };

		Self {
			position: Vector3 { x, y, z }
		}
	}

	fn make_construction(&self) -> SimplePhysicsConstruction {
		SimplePhysicsConstruction {
			climbing_property: None,
			velocity_info: None,
			motion_type: None,
			position_rotation_info: Some(PositionRotationInfo {
				position: self.position,
				rotation: Quaternion {
					x: 0.0,
					y: 0.7334349751472473,
					z: 0.0,
					w: 0.6797596216201782,
				},
			}),
		}
	}

	fn make_serialization(&self) -> SimplePhysicsSerialization {
		SimplePhysicsSerialization {
			velocity_info: None,
			motion_type: None,
			position_rotation_info: None,
		}
	}
}
