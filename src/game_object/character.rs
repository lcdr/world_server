use lu_packets::{
	lu,
	raknet::client::replica::character::{CharacterConstruction, CharacterProtocol, CharacterSerialization, GameActivity, GmPvpInfo, SocialInfo, TransitionState},
	world::LuNameValue,
	world::gm::server::{GameMessage as ServerGM, ParseChatMessage},
};

use crate::state::Connection;
use crate::state::State;
use super::{InternalComponent, GameObject};

pub struct CharacterComponent {

}

impl CharacterComponent {
	fn on_parse_chat_message(&mut self, msg: &ParseChatMessage, game_object: &mut GameObject, state: &mut State, conn: &mut Connection) {
		use lu_packets::common::LuStrExt;
		let string = msg.string.to_string();

		if string.starts_with("/") {
			dbg!(msg);
			crate::commands::on_chat_command(state, &string, game_object, conn);
		}
	}
}

impl InternalComponent for CharacterComponent {
	type ComponentProtocol = CharacterProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
	}

	fn make_construction(&self) -> CharacterConstruction {
		CharacterConstruction {
			claim_code_1: None,
			claim_code_2: None,
			claim_code_3: None,
			claim_code_4: None,
			hair_color: 11,
			hair_style: 6,
			torso_color: 1,
			legs_color: 6,
			torso_decal: 24,
			eyebrows_style: 38,
			eyes_style: 22,
			mouth_style: 24,
			account_id: 104116,
			last_logout: 0,
			prop_mod_last_display_time: 0,
			u_score: 0,
			is_free_trial: false,
			total_currency_collected: 0,
			total_bricks_collected: 0,
			total_smashables_smashed: 0,
			total_quickbuilds_completed: 0,
			total_enemies_smashed: 0,
			total_rockets_used: 0,
			total_missions_completed: 0,
			total_pets_tamed: 0,
			total_imagination_powerups_collected: 0,
			total_life_powerups_collected: 0,
			total_armor_powerups_collected: 0,
			total_distance_traveled: 0,
			times_smashed_count: 0,
			total_damage_taken: 0,
			total_damage_healed: 0,
			total_armor_repaired: 0,
			total_imagination_restored: 0,
			total_imagination_used: 0,
			total_distance_driven: 0,
			total_time_airborne_in_a_race_car: 0,
			total_racing_imagination_powerups_collected: 0,
			total_racing_imagination_crates_smashed: 0,
			total_racing_car_boosts_activated: 0,
			total_racing_wrecks: 0,
			total_racing_smashables_smashed: 0,
			total_races_finished: 0,
			total_first_place_race_finishes: 0,
			transition_state: TransitionState::None,
			gm_pvp_info: Some(GmPvpInfo {
				pvp_enabled: false,
				is_gm: false,
				gm_level: 0,
				editor_enabled: false,
				editor_level: 0,
			}),
			current_activity: Some(GameActivity::None),
			social_info: Some(SocialInfo {
				guild_id: 0,
				guild_name: lu!(""),
				is_lego_club_member: false,
			}),
		}
	}

	fn make_serialization(&self) -> CharacterSerialization {
		CharacterSerialization {
			gm_pvp_info: None,
			current_activity: None,
			social_info: None,
		}
	}

	fn on_game_message(&mut self, msg: &ServerGM, game_object: &mut GameObject, state: &mut State, conn: &mut Connection) {
		match msg {
			ServerGM::ParseChatMessage(x) => self.on_parse_chat_message(x, game_object, state, conn),
			_ => {}
		}
	}
}
