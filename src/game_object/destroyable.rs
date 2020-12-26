use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		destroyable::{DestroyableConstruction, StatsInfo, StatusImmunityInfo},
	},
};

use super::Component;

pub struct DestroyableComponent {

}

impl Component for DestroyableComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(DestroyableConstruction {
			status_immunity_info: Some(StatusImmunityInfo {
				immune_to_basic_attack: 0,
				immune_to_damage_over_time: 0,
				immune_to_knockback: 0,
				immune_to_interrupt: 0,
				immune_to_speed: 0,
				immune_to_imagination_gain: 0,
				immune_to_imagination_loss: 0,
				immune_to_quickbuild_interrupt: 0,
				immune_to_pull_to_point: 0,
			}),
			stats_info: Some(StatsInfo {
				cur_health: 4,
				max_health: 4.0,
				cur_armor: 0,
				max_armor: 0.0,
				cur_imag: 0,
				max_imag: 0.0,
				damage_absorption_points: 0,
				immunity: true,
				is_gm_immune: false,
				is_shielded: false,
				actual_max_health: 4.0,
				actual_max_armor: 0.0,
				actual_max_imag: 0.0,
				factions: vec![1].into(),
				is_dead: false,
				is_smashed: false,
				smashable_info: None,
			}),
			is_on_a_threat_list: Some(false),
		})
	}
}
