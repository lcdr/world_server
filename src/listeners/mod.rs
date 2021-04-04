//! Message listeners responsible for the behavior of the server in response to incoming messages.
mod chara;

use std::io::{Result as Res};

use lu_packets::{
	lu, lnv,
	chat::client::GeneralChatMessage as ClientChatMessage,
	world::client::{ChatModerationString, CreateCharacter},
	world::gm::client::GameMessage as ClientGM,
	world::gm::server::{SubjectGameMessage as ServerSGM},
	world::server::{GeneralChatMessage as ServerChatMessage, LevelLoadComplete, PositionUpdate, StringCheck, WorldMessage},
};

use crate::state::{AccountInfo, Connection, State};

use self::chara::{on_char_create_req, on_char_del_req, on_char_list_req, on_char_login_req};

pub fn on_validated_msg(state: &mut State, msg: &WorldMessage, acc_info: &mut AccountInfo, conn: &mut Connection) -> Res<()> {
	use lu_packets::world::server::WorldMessage::*;
	match msg {
		CharacterListRequest        => on_char_list_req(state, acc_info, conn),
		CharacterCreateRequest(msg) => on_char_create_req(state, msg, acc_info, conn),
		CharacterLoginRequest(msg)  => on_char_login_req(state, msg, acc_info, conn),
		CharacterDeleteRequest(msg) => on_char_del_req(state, msg, acc_info, conn),
		GeneralChatMessage(msg)     => on_general_chat_msg(state, msg, acc_info, conn),
		SubjectGameMessage(msg)     => on_subject_game_msg(state, msg, conn),
		LevelLoadComplete(msg)      => on_level_load_complete(state, msg, acc_info, conn),
		PositionUpdate(msg)         => on_position_update(state, msg, acc_info, conn),
		StringCheck(msg)            => on_string_check(state, msg, conn),
		_                           => { println!("Unrecognized packet: {:?}", msg); Ok(()) },
	}
}

fn on_general_chat_msg(state: &mut State, msg: &ServerChatMessage, acc_info: &AccountInfo, conn: &mut Connection) -> Res<()> {
	state.with_char(acc_info, |_state, sender| {
		conn.broadcast(ClientChatMessage {
			chat_channel: msg.chat_channel,
			sender: sender.object_id(),
			sender_name: lu!(""),
			source_id: msg.source_id,
			sender_gm_level: 0,
			message: msg.message.clone().into(),
		})
	})
}

fn on_level_load_complete(state: &mut State, _msg: &LevelLoadComplete, acc_info: &mut AccountInfo, conn: &mut Connection) -> Res<()> {
	let chara = state.spawn_player(acc_info).unwrap();

	let mut xml = String::new();
	chara.write_xml(&mut xml).unwrap();


	let obj_id = chara.object_id();
	let name = &format!("{}", obj_id)[..];

	let chardata = CreateCharacter { data: lnv! {
		"objid": obj_id,
		"template": 1i32,
		"name": name,
		"xmlData": &xml[..],
	}};
	conn.send(chardata)?;

	for game_object in state.all_game_objects() {
		let replica = game_object.make_construction();
		conn.broadcast(replica)?;
	}

	state.with_game_object(obj_id, |_state, chara| {
		let serverdone = chara.make_sgm(ClientGM::ServerDoneLoadingAllObjects);
		conn.send(serverdone)?;

		let playerready = chara.make_sgm(ClientGM::PlayerReady);
		conn.send(playerready)?;

		let postload = chara.make_sgm(ClientGM::RestoreToPostLoadStats);
		conn.send(postload)
	})
}

fn on_subject_game_msg(state: &mut State, msg: &ServerSGM, conn: &mut Connection) -> Res<()> {
	state.with_game_object(msg.subject_id, |state, game_object| {
		game_object.on_game_message(&msg.message, state, conn)
	})
}

fn on_position_update(state: &mut State, msg: &PositionUpdate, acc_info: &AccountInfo, conn: &mut Connection) -> Res<()> {
	state.with_char(acc_info, |_state, game_object| {
		game_object.run_service_mut(&msg.frame_stats)?;
		let ser = game_object.make_serialization();
		conn.broadcast(ser)
	})
}

fn on_string_check(_state: &State, msg: &StringCheck, conn: &mut Connection) -> Res<()> {
	let resp = ChatModerationString {
		request_id: msg.request_id,
		chat_mode: msg.chat_mode,
		whisper_name: lu!(""),
		spans: vec![],
	};
	conn.send(resp)
}
