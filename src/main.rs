#![recursion_limit="40"]
#[macro_use]
extern crate diesel;

mod listeners;
mod models;
mod schema;

use base_server::{create_tls_config, load_config, server::Server};
use lu_packets::{
	world::client::Message as OutMessage,
	world::server::Message as IncMessage,
};

use crate::listeners::MsgCallback;

/// Runs the server.
fn main() {
	let config = load_config();
	let tls_config = create_tls_config(config.tls);
	let mut listener = MsgCallback::new(&config.db.path);
	let mut server = Server::<IncMessage, OutMessage, _>::new("0.0.0.0:10000", tls_config, |i, o| MsgCallback::on_msg(&mut listener, i, o)).unwrap();
	println!("Started up");
	server.run();
}
