//! Message listeners for Character selection, creation, deletion, and login.
use std::convert::TryInto;
use std::io::{Error, ErrorKind::Other, Result as Res};

use diesel::prelude::*;
use diesel::dsl::{delete, insert_into};

use lu_packets::{
	world::{Vector3, ZoneId},
	world::client::{CharListChar, CharacterListResponse, CharacterCreateResponse, CharacterDeleteResponse, InstanceType, LoadStaticZone},
	world::server::{CharacterCreateRequest, CharacterDeleteRequest, CharacterLoginRequest},
};

use crate::state::{AccountInfo, Connection, State};
use crate::models::Character;

pub fn on_char_list_req(state: &State, acc_info: &AccountInfo, conn: &mut Connection) -> Res<()> {
	use crate::schema::characters::dsl::{characters, username};

	let chars: Vec<Character> = characters
	.filter(username.eq(&acc_info.username()))
	.load(state.db()).expect("Error loading characters");
	let mut list_chars = vec![];

	for chara in chars {
		list_chars.push(CharListChar {
			obj_id: (chara.id as u64) | (1 << 60),
			char_name: (&*chara.name).try_into().unwrap(),
			pending_name: "".try_into().unwrap(),
			requires_rename: false,
			is_free_trial: false,
			torso_color: chara.torso_color as u32,
			legs_color: chara.legs_color as u32,
			hair_style: chara.hair_style as u32,
			hair_color: chara.hair_color as u32,
			eyebrows_style: chara.eyebrows_style as u32,
			eyes_style: chara.eyes_style as u32,
			mouth_style: chara.mouth_style as u32,
			last_location: ZoneId { map_id: chara.world_zone as u16, instance_id: chara.world_instance as u16, clone_id: chara.world_clone as u32 },
			equipped_items: vec![].into(),
		});
	}

	conn.send(CharacterListResponse {
		selected_char: 0,
		chars: list_chars,
	})
}

pub fn on_char_create_req(state: &mut State, msg: &CharacterCreateRequest, acc_info: &AccountInfo, conn: &mut Connection) -> Res<()> {
	use crate::schema::characters::dsl::{characters};

	let new_char = Character {
		id: state.new_persistent_id() as i32,
		username: acc_info.username().to_string(),
		name: String::from(&msg.char_name),
		torso_color: msg.torso_color as i32,
		legs_color: msg.legs_color as i32,
		hair_style: msg.hair_style as i32,
		hair_color: msg.hair_color as i32,
		eyebrows_style: msg.eyebrows_style as i32,
		eyes_style: msg.eyes_style as i32,
		mouth_style: msg.mouth_style as i32,
		world_zone: 0,
		world_instance: 0,
		world_clone: 0,
	};

	if let Err(e) = insert_into(characters)
	.values(&new_char)
	.execute(state.db()) {
		conn.send(CharacterCreateResponse::GeneralFailure)?;
		return Err(Error::new(Other, format!("Error saving character: {}", e)));
	}

	conn.send(CharacterCreateResponse::Success)?;
	on_char_list_req(state, acc_info, conn)
}

pub fn on_char_login_req(_state: &State, _msg: &CharacterLoginRequest, _acc_info: &AccountInfo, conn: &mut Connection) -> Res<()> {
	let lsz = LoadStaticZone {
		zone_id: ZoneId { map_id: 1100, instance_id: 0, clone_id: 0 },
		map_checksum: 0x49525511,
		player_position: Vector3::ZERO,
		instance_type: InstanceType::Public,
	};

	conn.send(lsz)
}

pub fn on_char_del_req(state: &State, msg: &CharacterDeleteRequest, acc_info: &AccountInfo, conn: &mut Connection) -> Res<()> {
	use crate::schema::characters::dsl::{characters, id, username};

	let success = delete(characters
	.filter(username.eq(&acc_info.username()))
	.filter(id.eq(msg.char_id as i32)))
	.execute(state.db()).is_ok();

	if !success {
		eprintln!("Error deleting character {} from user {}", msg.char_id, acc_info.username());
	}

	conn.send(CharacterDeleteResponse { success })
}
