//! Message listeners responsible for the behavior of the server in response to incoming messages.
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::SocketAddr;

use diesel::prelude::*;
use diesel::dsl::{delete, insert_into};
use rusqlite::Connection as RusqliteConnection;

use lu_packets::{
	lu, lnv,
	common::ObjId,
	general::client::DisconnectNotify,
	world::{Lot, LuNameValue, Vector3, ZoneId},
	world::client::{CharListChar, CharacterListResponse, CharacterCreateResponse, CharacterDeleteResponse, ChatModerationString, CreateCharacter, InstanceType, LoadStaticZone, Message as OutMessage},
	world::gm::client::GameMessage as ClientGM,
	world::gm::server::{SubjectGameMessage as ServerSGM},
	world::server::{CharacterCreateRequest, CharacterDeleteRequest, CharacterLoginRequest, ClientValidation, LevelLoadComplete, Message as IncMessage, PositionUpdate, StringCheck, WorldMessage},
};
use lu_packets::common::ServiceId;
use base_server::listeners::{on_conn_req, on_internal_ping, on_handshake};
use base_server::server::Context as C;

use crate::game_object::GameObject;
use crate::models::Character;
pub type Context = C<IncMessage, OutMessage>;

pub struct AccountInfo {
	username: String,
	active_character_id: ObjId,
}

pub struct MsgCallback {
	validated: HashMap<SocketAddr, AccountInfo>,
	game_objects: HashMap<ObjId, GameObject>,
	current_spawned_id: ObjId,
	current_persistent_id: ObjId,
	current_network_id: u16,
	cdclient: RusqliteConnection,
	/// Connection to the users DB.
	conn: SqliteConnection,
}

impl MsgCallback {
	/// Creates a new callback connecting to the DB at the provided path.
	pub fn new(cdclient_path: &str, db_path: &str) -> Self {
		let cdclient = RusqliteConnection::open(cdclient_path).unwrap();
		let conn = SqliteConnection::establish(db_path).unwrap();

		const BITS_PERSISTENT: ObjId = 1 << 60;
		const BITS_LOCAL: ObjId = 1 << 46;
		const BITS_SPAWNED: ObjId = 1 << 58 | BITS_LOCAL;

		Self {
			validated: HashMap::new(),
			game_objects: HashMap::new(),
			current_spawned_id: BITS_SPAWNED,
			current_persistent_id: BITS_PERSISTENT,
			current_network_id: 0,
			cdclient,
			conn,
		}
	}

	/// Dispatches to the various handlers depending on message type.
	pub fn on_msg(&mut self, msg: &IncMessage, ctx: &mut Context) {
		use lu_packets::raknet::server::Message::{InternalPing, ConnectionRequest, NewIncomingConnection, UserMessage};
		use lu_packets::world::server::{
			LuMessage::{General, World},
			GeneralMessage::Handshake,
			WorldMessage::ClientValidation,
		};
		match msg {
			InternalPing(msg)                         => on_internal_ping::<IncMessage, OutMessage>(msg, ctx),
			ConnectionRequest(msg)                    => on_conn_req::<IncMessage, OutMessage>(msg, ctx),
			NewIncomingConnection(msg)                => { dbg!(msg); },
			UserMessage(General(Handshake(msg)))      => on_handshake::<IncMessage, OutMessage>(msg, ctx, ServiceId::World),
			UserMessage(World(ClientValidation(msg))) => self.on_client_val(msg, ctx),
			UserMessage(World(msg))                   => self.on_restricted_msg(msg, ctx),
			_                                         => { dbg!("do NOT contact me with unsolicited offers or services"); },
		}
	}

	fn on_client_val(&mut self, cli_val: &ClientValidation, ctx: &mut Context) {
		let username = String::from(&cli_val.username);
		let session_key = String::from(&cli_val.session_key);
		let resp = minreq::get(format!("http://localhost:21835/verify/{}/{}", username, session_key)).send().unwrap();
		if resp.status_code != 200 {
			eprintln!("Error {} when trying to verify {} {} with the auth server!", resp.status_code, username, session_key);
			ctx.send(DisconnectNotify::UnknownServerError).unwrap();
			ctx.close_conn();
			return;
		}
		if resp.as_bytes() != b"1" {
			println!("Login attempt from {} with invalid key {}!", username, session_key);
			ctx.send(DisconnectNotify::InvalidSessionKey).unwrap();
			ctx.close_conn();
			return;
		}
		let peer_addr = ctx.peer_addr().unwrap();
		self.validated.insert(peer_addr, AccountInfo { username, active_character_id: 0 });
	}

	fn on_restricted_msg(&mut self, msg: &WorldMessage, ctx: &mut Context) {
		let addr = ctx.peer_addr().unwrap();
		let mut acc_info = match self.validated.remove(&addr) {
			None =>  {
			println!("Restricted packet from unvalidated client!");
			ctx.send(DisconnectNotify::InvalidSessionKey).unwrap();
			ctx.close_conn();
			return;
			}
			Some(info) => info,
		};

		use lu_packets::world::server::WorldMessage::*;
		match msg {
			CharacterListRequest        => self.on_char_list_req(&acc_info, ctx),
			CharacterCreateRequest(msg) => self.on_char_create_req(msg, &acc_info, ctx),
			CharacterLoginRequest(msg)  => self.on_char_login_req(msg, &acc_info, ctx),
			CharacterDeleteRequest(msg) => self.on_char_del_req(msg, &acc_info, ctx),
			SubjectGameMessage(msg)     => self.on_subject_game_msg(msg, ctx),
			LevelLoadComplete(msg)      => self.on_level_load_complete(msg, &mut acc_info, ctx),
			PositionUpdate(msg)         => self.on_position_update(msg, &acc_info, ctx),
			StringCheck(msg)            => self.on_string_check(msg, ctx),
			_                           => { println!("Unrecognized packet: {:?}", msg); },
		}

		self.validated.insert(addr, acc_info);
	}

	fn on_char_list_req(&self, acc_info: &AccountInfo, ctx: &mut Context) {
		use crate::schema::characters::dsl::{characters, username};

		let chars: Vec<Character> = characters
		.filter(username.eq(&acc_info.username))
		.load(&self.conn).expect("Error loading characters");
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

		ctx.send(CharacterListResponse {
			selected_char: 0,
			chars: list_chars,
		}).unwrap()
	}

	fn on_char_create_req(&self, msg: &CharacterCreateRequest, acc_info: &AccountInfo, ctx: &mut Context) {
		use crate::schema::characters::dsl::{characters};

		let new_char = Character {
			id: 0, // good id
			username: acc_info.username.to_string(),
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
		.execute(&self.conn) {
			eprintln!("Error saving character: {}", e);
			ctx.send(CharacterCreateResponse::GeneralFailure).unwrap();
			return;
		}

		ctx.send(CharacterCreateResponse::Success).unwrap();
		self.on_char_list_req(acc_info, ctx);
	}

	fn on_char_login_req(&self, _msg: &CharacterLoginRequest, _acc_info: &AccountInfo, ctx: &mut Context) {
		let lsz = LoadStaticZone {
			zone_id: ZoneId { map_id: 1100, instance_id: 0, clone_id: 0 },
			map_checksum: 0x49525511,
			player_position: Vector3::ZERO,
			instance_type: InstanceType::Public,
		};

		ctx.send(lsz).unwrap();
	}

	fn on_char_del_req(&self, msg: &CharacterDeleteRequest, acc_info: &AccountInfo, ctx: &mut Context) {
		use crate::schema::characters::dsl::{characters, id, username};

		let success = delete(characters
		.filter(username.eq(&acc_info.username))
		.filter(id.eq(msg.char_id as i32)))
		.execute(&self.conn).is_ok();

		if !success {
			eprintln!("Error deleting character {} from user {}", msg.char_id, acc_info.username);
		}

		ctx.send(CharacterDeleteResponse { success }).unwrap();
	}

	fn on_level_load_complete(&mut self, _msg: &LevelLoadComplete, acc_info: &mut AccountInfo, ctx: &mut Context) {
		let chara = self.spawn_player();
		let obj_id = chara.object_id();

		acc_info.active_character_id = obj_id;

		let mut xml = String::new();
		chara.write_xml(&mut xml).unwrap();


		let name = &format!("{}", obj_id)[..];

		let chardata = CreateCharacter { data: lnv! {
			"objid": obj_id,
			"template": 1i32,
			"name": name,
			"xmlData": &xml[..],
		}};
		ctx.send(chardata).unwrap();

		for game_object in self.game_objects.values() {
			let replica = game_object.make_construction();
			ctx.broadcast(replica).unwrap();
		}

		let chara = &self.game_objects[&obj_id];

		let serverdone = chara.make_sgm(ClientGM::ServerDoneLoadingAllObjects);
		ctx.send(serverdone).unwrap();

		let playerready = chara.make_sgm(ClientGM::PlayerReady);
		ctx.send(playerready).unwrap();

		let postload = chara.make_sgm(ClientGM::RestoreToPostLoadStats);
		ctx.send(postload).unwrap();
	}

	fn on_subject_game_msg(&mut self, msg: &ServerSGM, ctx: &mut Context) {
		self.with_game_object(msg.subject_id, |server, game_object| {
			game_object.on_game_message(&msg.message, server, ctx).unwrap();
		});
	}

	fn on_position_update(&mut self, msg: &PositionUpdate, acc_info: &AccountInfo, _ctx: &mut Context) {
		self.with_game_object(acc_info.active_character_id, |_server, game_object| {
			game_object.run_service_mut(&msg.frame_stats);
		});
	}

	fn on_string_check(&self, msg: &StringCheck, ctx: &mut Context) {
		let resp = ChatModerationString {
			request_id: msg.request_id,
			chat_mode: msg.chat_mode,
			whisper_name: lu!(""),
			spans: vec![],
		};
		ctx.send(resp).unwrap();
	}

	fn with_game_object<F: FnOnce(&mut MsgCallback, &mut GameObject)>(&mut self, obj_id: ObjId, callback: F) {
		let mut game_object = match self.game_objects.remove(&obj_id) {
			Some(x) => x,
			None => {
				eprintln!("Game object {} does not exist!", obj_id);
				return;
			}
		};
		callback(self, &mut game_object);
		self.game_objects.insert(obj_id, game_object);
	}

	fn new_spawned_id(&mut self) -> ObjId {
		self.current_spawned_id += 1;
		return self.current_spawned_id;
	}

	fn new_persistent_id(&mut self) -> ObjId {
		self.current_persistent_id += 1;
		return self.current_persistent_id;
	}

	fn new_network_id(&mut self) -> u16 {
		self.current_network_id += 1;
		return self.current_network_id;
	}

	fn spawn_player(&mut self) -> &GameObject {
		self.spawn_internal(true, 1, &lnv!{})
	}

	pub fn spawn(&mut self, lot: Lot, config: &LuNameValue) -> &mut GameObject {
		self.spawn_internal(false, lot, config)
	}

	fn spawn_internal(&mut self, is_persistent: bool, lot: Lot, config: &LuNameValue) -> &mut GameObject {
		let network_id = self.new_network_id();
		let obj_id = if is_persistent { self.new_persistent_id() } else { self.new_spawned_id() };
		let game_object = GameObject::new(network_id, obj_id, lot, config, &self.cdclient);
		self.game_objects.insert(obj_id, game_object);
		self.game_objects.get_mut(&obj_id).unwrap()
	}
}
