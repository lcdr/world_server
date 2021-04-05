use std::io::Result as Res;

use lu_packets::{
	amf3, lnv, lu,
	chat::ChatChannel,
	chat::client::GeneralChatMessage as ClientChatMessage,
	world::gm::client::{SetJetPackMode, UiMessageServerToSingleClient},
};

use crate::game_object::GameObject;
use crate::state::{Connection, State};
use crate::services::{AddItem, GetPosition, GetRotation};

pub fn on_chat_command(state: &mut State, string: &str, sender: &mut GameObject, conn: &mut Connection) {
	let args: Vec<_> = string.split_whitespace().collect();
	let command = match &args[0][1..] {
		"additem"   => add_item_cmd,
		"gamestate" => send_gamestate_cmd,
		"jetpack"   => jetpack_cmd,
		"uidebug"   => send_uidebug_cmd,
		"toggle"    => send_toggle_scoreboard_cmd,
		"spawn"     => spawn_cmd,
		"dance"     => nop_cmd,
		_           => unknown_cmd,
	};

	if let Err(error) = command(state, sender, conn, &args) {
		conn.send(system_message(&format!("Error in command: {}", error))).unwrap();
	}
}

fn system_message(string: &str) -> ClientChatMessage {
	ClientChatMessage {
		chat_channel: ChatChannel::Local,
		sender: 0,
		sender_name: lu!(""),
		source_id: 0,
		sender_gm_level: 0,
		message: lu!(string),
	}
}

fn add_item_cmd(state: &mut State, sender: &mut GameObject, conn: &mut Connection, args: &Vec<&str>) -> Res<()> {
	if args.len() != 2 {
		return Ok(());
	}
	let lot = args[1].parse().unwrap();
	let mut add_item = AddItem { lot, state, conn };
	sender.run_service_mut(&mut add_item)
}

fn jetpack_cmd(_state: &mut State, sender: &mut GameObject, conn: &mut Connection, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(SetJetPackMode {
		bypass_checks: true,
		do_hover: false,
		use_jetpack: true,
		effect_id: 167,
		airspeed: 20.0,
		max_airspeed: 30.0,
		vert_vel: 1.5,
		warning_effect_id: -1,
	});
	conn.send(uimsg)
}

fn send_uidebug_cmd(_state: &mut State, sender: &mut GameObject, conn: &mut Connection, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {
			"visible": true,
		},
		message_name: lu!(b"ToggleUIDebugger"),
	});
	conn.send(uimsg)
}

fn send_gamestate_cmd(_state: &mut State, sender: &mut GameObject, conn: &mut Connection, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {
			"state": "Survival",
		},
		message_name: lu!(b"pushGameState"),
	});
	conn.send(uimsg)
}

fn send_toggle_scoreboard_cmd(_state: &mut State, sender: &mut GameObject, conn: &mut Connection, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {"visible": false},
		message_name: lu!(b"ToggleSurvivalScoreboard"),
	});
	conn.send(uimsg)?;
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {"visible": true},
		message_name: lu!(b"ToggleSurvivalScoreboard"),
	});
	conn.send(uimsg)?;
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {
			"iplayerName": "Allies",
			"itime": "200",
			"inextbestname": "Enemies",
			"inextbesttime": "321",
		},
		message_name: lu!(b"UpdateSurvivalScoreboard"),
	});
	conn.send(uimsg)
}

fn spawn_cmd(state: &mut State, sender: &mut GameObject, conn: &mut Connection, args: &Vec<&str>) -> Res<()> {
	if args.len() != 2 {
		return Ok(());
	}
	let lot = args[1].parse().unwrap();
	let mut get_pos = GetPosition::default();
	sender.run_service(&mut get_pos);
	let mut get_rot = GetRotation::default();
	sender.run_service(&mut get_rot);
	let config = lnv! {
		"position_x": get_pos.0.x,
		"position_y": get_pos.0.y,
		"position_z": get_pos.0.z,
		"rotation_x": get_rot.0.x,
		"rotation_y": get_rot.0.y,
		"rotation_z": get_rot.0.z,
		"rotation_w": get_rot.0.w,
	};
	let game_object = state.spawn(lot, &config)?;

	let replica = game_object.make_construction();
	conn.broadcast(replica)
}

fn nop_cmd(_state: &mut State, _sender: &mut GameObject, _conn: &mut Connection, _args: &Vec<&str>) -> Res<()> {
	Ok(())
}

fn unknown_cmd(_state: &mut State, _sender: &mut GameObject, conn: &mut Connection, _args: &Vec<&str>) -> Res<()> {
	conn.send(system_message("Unknown command."))
}
