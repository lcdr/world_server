//! Message listeners responsible for the behavior of the server in response to incoming messages.
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::SocketAddr;

use diesel::prelude::*;
use diesel::dsl::{delete, insert_into};

use lu_packets::{
	general::client::DisconnectNotify,
	world::{Vector3, ZoneId},
	world::client::{CharListChar, CharacterListResponse, CharacterCreateResponse, CharacterDeleteResponse, InstanceType, LoadStaticZone, Message as OutMessage},
	world::server::{CharacterCreateRequest, CharacterDeleteRequest, CharacterLoginRequest, ClientValidation, LevelLoadComplete, Message as IncMessage, WorldMessage},
};
use lu_packets::common::ServiceId;
use base_server::listeners::{on_conn_req, on_internal_ping, on_handshake};
use base_server::server::Context as C;

use crate::models::Character;
type Context = C<IncMessage, OutMessage>;

pub struct MsgCallback {
	validated: HashMap<SocketAddr, String>,
	/// Connection to the users DB.
	conn: SqliteConnection,
}

impl MsgCallback {
	/// Creates a new callback connecting to the DB at the provided path.
	pub fn new(db_path: &str) -> Self {
		let conn = SqliteConnection::establish(db_path).unwrap();
		Self { validated: HashMap::new(), conn }
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
			return;
		}
		if resp.as_bytes() != b"1" {
			println!("Login attempt from {} with invalid key {}!", username, session_key);
			return;
		}
		let peer_addr = ctx.peer_addr().unwrap();
		self.validated.insert(peer_addr, username);
	}

	pub fn on_restricted_msg(&self, msg: &WorldMessage, ctx: &mut Context) {
		dbg!(&msg);
		let username = match self.validated.get(&ctx.peer_addr().unwrap()) {
			None =>  {
			println!("Restricted packet from unvalidated client!");
			ctx.send(DisconnectNotify::InvalidSessionKey).unwrap();
			ctx.close_conn();
			return;
			}
			Some(u) => u,
		};

		use lu_packets::world::server::WorldMessage::*;
		match msg {
			CharacterListRequest        => self.on_char_list_req(&username, ctx),
			CharacterCreateRequest(msg) => self.on_char_create_req(msg, &username, ctx),
			CharacterLoginRequest(msg)  => self.on_char_login_req(msg, &username, ctx),
			CharacterDeleteRequest(msg) => self.on_char_del_req(msg, &username, ctx),
			LevelLoadComplete(msg)      => self.on_level_load_complete(msg, &username, ctx),
			_                           => { println!("Unrecognized packet: {:?}", msg); },
		}
	}

	fn on_char_list_req(&self, provided_username: &str, ctx: &mut Context) {
		use crate::schema::characters::dsl::{characters, username};

		let chars: Vec<Character> = characters
		.filter(username.eq(provided_username))
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
				eyebrow_style: chara.eyebrow_style as u32,
				eye_style: chara.eye_style as u32,
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

	fn on_char_create_req(&self, msg: &CharacterCreateRequest, username: &str, ctx: &mut Context) {
		use crate::schema::characters::dsl::{characters};

		let new_char = Character {
			id: 0, // good id
			username: username.to_string(),
			name: String::from(&msg.char_name),
			torso_color: msg.torso_color as i32,
			legs_color: msg.legs_color as i32,
			hair_style: msg.hair_style as i32,
			hair_color: msg.hair_color as i32,
			eyebrow_style: msg.eyebrow_style as i32,
			eye_style: msg.eye_style as i32,
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
		self.on_char_list_req(username, ctx);
	}

	fn on_char_login_req(&self, _msg: &CharacterLoginRequest, _username: &str, ctx: &mut Context) {
		let lsz = LoadStaticZone {
			zone_id: ZoneId { map_id: 1000, instance_id: 0, clone_id: 0 },
			map_checksum: 0x20b8087c,
			player_position: Vector3::ZERO,
			instance_type: InstanceType::Public,
		};

		ctx.send(lsz).unwrap();
	}

	fn on_char_del_req(&self, msg: &CharacterDeleteRequest, provided_username: &str, ctx: &mut Context) {
		use crate::schema::characters::dsl::{characters, id, username};

		let success = delete(characters
		.filter(username.eq(provided_username))
		.filter(id.eq(msg.char_id as i32)))
		.execute(&self.conn).is_ok();

		if !success {
			eprintln!("Error deleting character {} from user {}", msg.char_id, provided_username);
		}

		ctx.send(CharacterDeleteResponse { success }).unwrap();
	}

	fn on_level_load_complete(&self, _msg: &LevelLoadComplete, _username: &str, _ctx: &mut Context) {
		// just dab for now
		dbg!("dab");
	}
}
