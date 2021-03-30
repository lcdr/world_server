use lu_packets::{
	lu,
	raknet::client::replica::simple_physics::{PositionRotationInfo, SimplePhysicsConstruction, SimplePhysicsProtocol, SimplePhysicsSerialization},
	world::{LuNameValue, LnvValue, Quaternion, Vector3},
};

use crate::services::GameObjectService;
use super::InternalComponent;

pub struct SimplePhysicsComponent {
	position: Vector3,
	rotation: Quaternion,
}

impl InternalComponent for SimplePhysicsComponent {
	type ComponentProtocol = SimplePhysicsProtocol;

	fn new(config: &LuNameValue) -> Self {
		let pos_x = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_x")) { *x } else { 156.0 };
		let pos_y = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_y")) { *x } else { 380.0 };
		let pos_z = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_z")) { *x } else { -187.0 };

		let rot_x = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_x")) { *x } else { 0.0 };
		let rot_y = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_y")) { *x } else { 0.0 };
		let rot_z = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_z")) { *x } else { 0.0 };
		let rot_w = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_w")) { *x } else { 0.0 };

		Self {
			position: Vector3 { x: pos_x, y: pos_y, z: pos_z },
			rotation: Quaternion { x: rot_x, y: rot_y, z: rot_z, w: rot_w },
		}
	}

	fn make_construction(&self) -> SimplePhysicsConstruction {
		SimplePhysicsConstruction {
			climbing_property: None,
			velocity_info: None,
			motion_type: None,
			position_rotation_info: Some(PositionRotationInfo {
				position: self.position,
				rotation: self.rotation,
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

	fn run_service(&self, service: &mut GameObjectService) {
		match service {
			GameObjectService::GetPosition(x) => {
				x.0 = self.position;
			}
			GameObjectService::GetRotation(x) => {
				x.0 = self.rotation;
			}
			_ => {},
		}
	}
}
