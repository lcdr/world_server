use lu_packets::{
	lu,
	raknet::client::replica::controllable_physics::{ControllablePhysicsConstruction, ControllablePhysicsProtocol, ControllablePhysicsSerialization, FrameStats, FrameStatsTeleportInfo},
	world::{LuNameValue, LnvValue, Quaternion, Vector3},
};

use crate::services::{GameObjectService, GameObjectServiceMut};
use super::InternalComponent;

pub struct ControllablePhysicsComponent {
	position: Vector3,
	rotation: Quaternion,
	is_on_ground: bool,
	is_on_rail: bool,
	linear_velocity: Option<Vector3>,
	angular_velocity: Option<Vector3>,
}

impl InternalComponent for ControllablePhysicsComponent {
	type ComponentProtocol = ControllablePhysicsProtocol;

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
			is_on_ground: true,
			is_on_rail: false,
			linear_velocity: None,
			angular_velocity: None,
		}
	}

	fn make_construction(&self) -> ControllablePhysicsConstruction {
		ControllablePhysicsConstruction {
			jetpack_info: None,
			stun_immunity_info: None,
			cheat_info: None,
			unknown_1: None,
			unknown_2: None,
			frame_stats: Some(FrameStats {
				position: self.position,
				rotation: self.rotation,
				is_on_ground: self.is_on_ground,
				is_on_rail: self.is_on_rail,
				linear_velocity: self.linear_velocity,
				angular_velocity: self.angular_velocity,
				local_space_info: None,
			}),
		}
	}

	fn make_serialization(&self) -> ControllablePhysicsSerialization {
		ControllablePhysicsSerialization {
			cheat_info: None,
			unknown_1: None,
			unknown_2: None,
			frame_stats_teleport_info: Some(FrameStatsTeleportInfo {
				frame_stats: FrameStats {
					position: self.position,
					rotation: self.rotation,
					is_on_ground: self.is_on_ground,
					is_on_rail: self.is_on_rail,
					linear_velocity: self.linear_velocity,
					angular_velocity: self.angular_velocity,
					local_space_info: None,
				},
				is_teleporting: false,
			}),
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

	fn run_service_mut(&mut self, service: &mut GameObjectServiceMut) {
		match service {
			GameObjectServiceMut::SetFrameStats(frame_stats) => {
				self.position = frame_stats.position;
				self.rotation = frame_stats.rotation;
				self.is_on_ground = frame_stats.is_on_ground;
				self.is_on_rail = frame_stats.is_on_rail;
				self.linear_velocity = frame_stats.linear_velocity;
				self.angular_velocity = frame_stats.angular_velocity;
			},
			_ => {},
		}
	}
}
