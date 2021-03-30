use std::io::Result as Res;

use lu_packets::{
	amf3, lnv, lu,
	chat::ChatChannel,
	chat::client::GeneralChatMessage as ClientChatMessage,
	world::gm::client::{SetJetPackMode, UiMessageServerToSingleClient},
	world::server::GeneralChatMessage as ServerChatMessage,
};

use crate::game_object::GameObject;
use crate::listeners::{AccountInfo, Context, MsgCallback};
use crate::services::{GetPosition, GetRotation};

pub fn on_general_chat_msg(server: &mut MsgCallback, msg: &ServerChatMessage, acc_info: &AccountInfo, ctx: &mut Context) -> Res<()> {
	server.with_char(acc_info, |_server, sender| {
		ctx.broadcast(ClientChatMessage {
			chat_channel: msg.chat_channel,
			sender: sender.object_id(),
			sender_name: lu!(""),
			source_id: msg.source_id,
			sender_gm_level: 0,
			message: msg.message.clone().into(),
		})
	})
}

pub fn on_chat_command(server: &mut MsgCallback, string: &str, sender: &GameObject, ctx: &mut Context) {
	let args: Vec<_> = string.split_whitespace().collect();
	let command = match &args[0][1..] {
		"jetpack"   => jetpack_cmd,
		"uidebug"   => send_uidebug_cmd,
		"gamestate" => send_gamestate_cmd,
		"toggle"    => send_toggle_scoreboard_cmd,
		"spawn"     => spawn_cmd,
		_           => nop_cmd,
	};

	if let Err(error) = command(server, sender, ctx, &args) {
		ctx.send(system_message(&format!("Error in command: {}", error))).unwrap();
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

fn jetpack_cmd(_server: &mut MsgCallback, sender: &GameObject, ctx: &mut Context, _args: &Vec<&str>) -> Res<()> {
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
	ctx.send(uimsg)
}

fn send_uidebug_cmd(_server: &mut MsgCallback, sender: &GameObject, ctx: &mut Context, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {
			"visible": true,
		},
		message_name: lu!(b"ToggleUIDebugger"),
	});
	ctx.send(uimsg)
}

fn send_gamestate_cmd(_server: &mut MsgCallback, sender: &GameObject, ctx: &mut Context, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {
			"state": "Survival",
		},
		message_name: lu!(b"pushGameState"),
	});
	ctx.send(uimsg)
}

fn send_toggle_scoreboard_cmd(_server: &mut MsgCallback, sender: &GameObject, ctx: &mut Context, _args: &Vec<&str>) -> Res<()> {
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {"visible": false},
		message_name: lu!(b"ToggleSurvivalScoreboard"),
	});
	ctx.send(uimsg)?;
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {"visible": true},
		message_name: lu!(b"ToggleSurvivalScoreboard"),
	});
	ctx.send(uimsg)?;
	let uimsg = sender.make_sgm(UiMessageServerToSingleClient {
		args: amf3! {
			"iplayerName": "Allies",
			"itime": "200",
			"inextbestname": "Enemies",
			"inextbesttime": "321",
		},
		message_name: lu!(b"UpdateSurvivalScoreboard"),
	});
	ctx.send(uimsg)
}

fn spawn_cmd(server: &mut MsgCallback, sender: &GameObject, ctx: &mut Context, args: &Vec<&str>) -> Res<()> {
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
	let game_object = server.spawn(lot, &config)?;

	let replica = game_object.make_construction();
	ctx.broadcast(replica)
}

fn nop_cmd(_server: &mut MsgCallback, _sender: &GameObject, _ctx: &mut Context, _args: &Vec<&str>) -> Res<()> {
	Ok(())
}
