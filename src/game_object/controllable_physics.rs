use lu_packets::{
	lu,
	raknet::client::replica::{ComponentConstruction,
		controllable_physics::{ControllablePhysicsConstruction, FrameStats},
	},
	world::{LuNameValue, LnvValue, Quaternion, Vector3},
};

use crate::services::{GameObjectService, GameObjectServiceMut};
use super::Component;

pub struct ControllablePhysicsComponent {
	position: Vector3,
	rotation: Quaternion,
}

impl Component for ControllablePhysicsComponent {
	fn new(config: &LuNameValue) -> Box<dyn Component> {
		let pos_x = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_x")) { *x } else { 156.0 };
		let pos_y = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_y")) { *x } else { 380.0 };
		let pos_z = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_z")) { *x } else { -187.0 };

		let rot_x = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_x")) { *x } else { 0.0 };
		let rot_y = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_y")) { *x } else { 0.0 };
		let rot_z = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_z")) { *x } else { 0.0 };
		let rot_w = if let Some(LnvValue::F32(x)) = config.get(&lu!("rotation_w")) { *x } else { 0.0 };

		Box::new(Self {
			position: Vector3 { x: pos_x, y: pos_y, z: pos_z },
			rotation: Quaternion { x: rot_x, y: rot_y, z: rot_z, w: rot_w },
		})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(ControllablePhysicsConstruction {
			jetpack_info: None,
			stun_immunity_info: None,
			cheat_info: None,
			unknown_1: None,
			unknown_2: None,
			frame_stats: Some(FrameStats {
				position: self.position,
				rotation: self.rotation,
				is_on_ground: true,
				is_on_rail: false,
				linear_velocity: None,
				angular_velocity: None,
				local_space_info: None,
			}),
		})
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

	fn run_service_mut(&mut self, service: &mut GameObjectServiceMut) {
		match service {
			GameObjectServiceMut::SetFrameStats(frame_stats) => {
				self.position = frame_stats.position;
				self.rotation = frame_stats.rotation;
			},
			_ => {},
		}
	}
}
