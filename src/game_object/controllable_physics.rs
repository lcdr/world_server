use lu_packets::{
	lu,
	raknet::client::replica::{ComponentConstruction,
		controllable_physics::{ControllablePhysicsConstruction, FrameStats},
	},
	world::{LuNameValue, LnvValue, Quaternion, Vector3},
};

use crate::services::GameObjectService;
use super::Component;

pub struct ControllablePhysicsComponent {
	position: Vector3,
}

impl Component for ControllablePhysicsComponent {
	fn new(config: &LuNameValue) -> Box<dyn Component> {
		let x = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_x")) { *x } else { 156.0 };
		let y = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_y")) { *x } else { 380.0 };
		let z = if let Some(LnvValue::F32(x)) = config.get(&lu!("position_z")) { *x } else { -187.0 };

		Box::new(Self {
			position: Vector3 { x, y, z }
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
				position: Vector3 {
					x: self.position.x,
					y: self.position.y,
					z: self.position.z,
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

	fn run_service(&self, service: &mut GameObjectService) {
		match service {
			GameObjectService::GetPosition(x) => {
				x.position.x = self.position.x;
				x.position.y = self.position.y;
				x.position.z = self.position.z;
			}
		}
	}
}
