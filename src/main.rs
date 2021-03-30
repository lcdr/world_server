#![recursion_limit="40"]
#[macro_use]
extern crate diesel;

mod chat;
mod game_object;
mod listeners;
mod models;
mod schema;
mod services;

use serde::Deserialize;

use base_server::{create_tls_config, TlsConf, server::Server};
use lu_packets::{
	world::client::Message as OutMessage,
	world::server::Message as IncMessage,
};

use crate::listeners::MsgCallback;

#[derive(Deserialize)]
struct Config {
	db: DbConf,
	cdclient: CdclientConf,
	tls: TlsConf,
}

#[derive(Deserialize)]
struct DbConf {
	path: String,
}

#[derive(Deserialize)]
struct CdclientConf {
	path: String,
}

fn load_config() -> Config {
	let mut exe_path = std::env::current_exe().expect("program location unknown");
	exe_path.pop();
	exe_path.push("config.toml");
	let config = std::fs::read_to_string(exe_path).expect("cannot open config file config.toml");
	let config: Config = toml::from_str(&config).expect("config file parsing error");

	config
}

/// Runs the server.
fn main() {
	let config = load_config();
	let tls_config = create_tls_config(config.tls);
	let mut listener = MsgCallback::new(&config.cdclient.path, &config.db.path);
	let mut server = Server::<IncMessage, OutMessage, _>::new("0.0.0.0:10000", tls_config, |i, o| listener.on_msg(i, o)).unwrap();
	println!("Started up");
	server.run();
}
