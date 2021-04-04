//! State providing context for listeners.
use std::collections::{hash_map, HashMap};
use std::io::Result as Res;
use std::net::SocketAddr;

use diesel::prelude::SqliteConnection;
use rusqlite::Connection as RusqliteConnection;

use lu_packets::{
	lnv,
	common::ObjId,
	general::client::DisconnectNotify,
	world::{Lot, LuNameValue},
	world::client::Message as OutMessage,
	world::server::{ClientValidation, Message as IncMessage, WorldMessage},
};
use lu_packets::common::ServiceId;
use base_server::listeners::{on_conn_req, on_internal_ping, on_handshake};
use base_server::server::Context as C;

use crate::game_object::GameObject;
use crate::listeners::on_validated_msg;
pub type Connection = C<IncMessage, OutMessage>;

pub struct AccountInfo {
	username: String,
	active_character_id: ObjId,
}

impl AccountInfo {
	pub fn username(&self) -> &String {
		&self.username
	}
}

pub struct State {
	validated: HashMap<SocketAddr, AccountInfo>,
	game_objects: HashMap<ObjId, GameObject>,
	current_spawned_id: ObjId,
	current_persistent_id: ObjId,
	current_network_id: u16,
	cdclient: RusqliteConnection,
	/// Connection to the users DB.
	db: SqliteConnection,
}

impl State {
	/// Creates a new callback connecting to the DB at the provided path.
	pub fn new(cdclient_path: &str, db_path: &str) -> Self {
		use diesel::Connection;

		let cdclient = RusqliteConnection::open(cdclient_path).unwrap();
		let db = SqliteConnection::establish(db_path).unwrap();

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
			db,
		}
	}

	/// Dispatches to the various handlers depending on message type.
	pub fn on_msg(&mut self, msg: &IncMessage, conn: &mut Connection) {
		use lu_packets::raknet::server::Message::{InternalPing, ConnectionRequest, NewIncomingConnection, UserMessage};
		use lu_packets::world::server::{
			LuMessage::{General, World},
			GeneralMessage::Handshake,
			WorldMessage::ClientValidation,
		};
		match msg {
			InternalPing(msg)                         => on_internal_ping::<IncMessage, OutMessage>(msg, conn),
			ConnectionRequest(msg)                    => on_conn_req::<IncMessage, OutMessage>(msg, conn),
			NewIncomingConnection(msg)                => { dbg!(msg); Ok(()) },
			UserMessage(General(Handshake(msg)))      => on_handshake::<IncMessage, OutMessage>(msg, conn, ServiceId::World),
			UserMessage(World(ClientValidation(msg))) => self.on_client_val(msg, conn),
			UserMessage(World(msg))                   => self.on_restricted_msg(msg, conn),
			_                                         => { dbg!("do NOT contact me with unsolicited offers or services"); Ok(()) },
		}.unwrap();
	}

	fn on_client_val(&mut self, cli_val: &ClientValidation, conn: &mut Connection) -> Res<()> {
		let username = String::from(&cli_val.username);
		let session_key = String::from(&cli_val.session_key);
		let resp = minreq::get(format!("http://localhost:21835/verify/{}/{}", username, session_key)).send().unwrap();
		if resp.status_code != 200 {
			eprintln!("Error {} when trying to verify {} {} with the auth server!", resp.status_code, username, session_key);
			conn.send(DisconnectNotify::UnknownServerError)?;
			conn.close_conn();
			return Ok(());
		}
		if resp.as_bytes() != b"1" {
			println!("Login attempt from {} with invalid key {}!", username, session_key);
			conn.send(DisconnectNotify::InvalidSessionKey)?;
			conn.close_conn();
			return Ok(());
		}
		let peer_addr = conn.peer_addr().unwrap();
		self.validated.insert(peer_addr, AccountInfo { username, active_character_id: 0 });
		Ok(())
	}

	fn on_restricted_msg(&mut self, msg: &WorldMessage, conn: &mut Connection) -> Res<()> {
		let addr = conn.peer_addr().unwrap();
		let mut acc_info = match self.validated.remove(&addr) {
			None => {
				println!("Restricted packet from unvalidated client!");
				conn.send(DisconnectNotify::InvalidSessionKey)?;
				conn.close_conn();
				return Ok(());
			}
			Some(info) => info,
		};
		on_validated_msg(self, msg, &mut acc_info, conn)?;
		self.validated.insert(addr, acc_info);
		Ok(())
	}

	pub fn db(&self) -> &SqliteConnection {
		&self.db
	}

	pub fn all_game_objects(&self) -> hash_map::Values<ObjId, GameObject> {
		self.game_objects.values()
	}

	pub fn with_game_object<F: FnOnce(&mut State, &mut GameObject) -> Res<()>>(&mut self, obj_id: ObjId, callback: F) -> Res<()> {
		let mut game_object = match self.game_objects.remove(&obj_id) {
			Some(x) => x,
			None => {
				eprintln!("Game object {} does not exist!", obj_id);
				return Ok(());
			}
		};
		callback(self, &mut game_object)?;
		self.game_objects.insert(obj_id, game_object);
		Ok(())
	}

	pub fn with_char<F: FnOnce(&mut State, &mut GameObject) -> Res<()>>(&mut self, acc_info: &AccountInfo, callback: F) -> Res<()> {
		self.with_game_object(acc_info.active_character_id, callback)
	}

	pub fn new_spawned_id(&mut self) -> ObjId {
		self.current_spawned_id += 1;
		return self.current_spawned_id;
	}

	pub fn new_persistent_id(&mut self) -> ObjId {
		self.current_persistent_id += 1;
		return self.current_persistent_id;
	}

	fn new_network_id(&mut self) -> u16 {
		self.current_network_id += 1;
		return self.current_network_id;
	}

	pub fn spawn_player(&mut self, acc_info: &mut AccountInfo) -> Res<&mut GameObject> {
		let chara = self.spawn_internal(true, 1, &lnv!{})?;
		acc_info.active_character_id = chara.object_id();
		Ok(chara)
	}

	pub fn spawn(&mut self, lot: Lot, config: &LuNameValue) -> Res<&mut GameObject> {
		self.spawn_internal(false, lot, config)
	}

	fn spawn_internal(&mut self, is_persistent: bool, lot: Lot, config: &LuNameValue) -> Res<&mut GameObject> {
		let network_id = self.new_network_id();
		let obj_id = if is_persistent { self.new_persistent_id() } else { self.new_spawned_id() };
		let game_object = GameObject::new(network_id, obj_id, lot, config, &self.cdclient)?;
		self.game_objects.insert(obj_id, game_object);
		Ok(self.game_objects.get_mut(&obj_id).unwrap())
	}
}
