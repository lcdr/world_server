use std::io::Result as Res;

use lu_packets::{
	amf3, lnv, lu,
	world::gm::client::{SetJetPackMode, UiMessageServerToSingleClient},
};

use crate::game_object::GameObject;
use crate::listeners::{Context, MsgCallback};
use crate::services::{GetPosition, GetRotation};

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

	command(server, sender, ctx, &args).unwrap();
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
	let config = lnv!(
		"position_x": get_pos.0.x,
		"position_y": get_pos.0.y,
		"position_z": get_pos.0.z,
		"rotation_x": get_rot.0.x,
		"rotation_y": get_rot.0.y,
		"rotation_z": get_rot.0.z,
		"rotation_w": get_rot.0.w,
	);
	let game_object = server.spawn(lot, &config);

	let replica = game_object.make_construction();
	ctx.broadcast(replica)
}

fn nop_cmd(_server: &mut MsgCallback, _sender: &GameObject, _ctx: &mut Context, _args: &Vec<&str>) -> Res<()> {
	Ok(())
}
