use std::io::Result as Res;

use lu_packets::{
	lnv,
	common::ObjId,
	raknet::client::replica::inventory::{EquippedItemInfo, InventoryConstruction, InventoryProtocol, InventorySerialization},
	world::{LuNameValue, Vector3},
	world::gm::InventoryType,
	world::gm::client::{AddItemToInventoryClientSync, LootType},
	world::gm::server::{GameMessage as ServerGM, EquipInventory},
};

use crate::services::GameObjectServiceMut;
use crate::state::{Connection, State};
use super::{GameObject, InternalComponent};

struct Item {
	object_id: ObjId,
}

pub struct InventoryComponent {
	items: Vec<Item>,
}

impl InventoryComponent {
	fn on_equip_inventory(&mut self, msg: &EquipInventory, game_object: &mut GameObject, state: &mut State, conn: &mut Connection) -> Res<()> {
		for item in &self.items {
			if item.object_id == msg.item_to_equip {

			}
		}
		Ok(())
	}
}

impl InternalComponent for InventoryComponent {
	type ComponentProtocol = InventoryProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self { items: vec![] }
	}

	fn make_construction(&self) -> InventoryConstruction {
		InventoryConstruction {
			equipped_items: Some(vec![
				EquippedItemInfo {
					id: 1152921510436607008,
					lot: 4106,
					subkey: None,
					count: Some(1),
					slot: None,
					inventory_type: None,
					extra_info: None,
					is_bound: true,
				},
				EquippedItemInfo {
					id: 1152921510436607009,
					lot: 2524,
					subkey: None,
					count: Some(1),
					slot: Some(1),
					inventory_type: None,
					extra_info: None,
					is_bound: true,
				},
			].into()),
			equipped_model_transforms: None,
		}
	}

	fn make_serialization(&self) -> InventorySerialization {
		InventorySerialization {
			equipped_items: Some(vec![
				EquippedItemInfo {
					id: 1152921510436607008,
					lot: 4106,
					subkey: None,
					count: Some(1),
					slot: None,
					inventory_type: None,
					extra_info: None,
					is_bound: true,
				},
				EquippedItemInfo {
					id: 1152921510436607009,
					lot: 2524,
					subkey: None,
					count: Some(1),
					slot: Some(1),
					inventory_type: None,
					extra_info: None,
					is_bound: true,
				},
			].into()),
			equipped_model_transforms: None,
		}
	}

	fn write_xml(&self, writer: &mut String) -> std::fmt::Result {
		use std::fmt::Write;
		write!(writer, "<inv><items><in t=\"0\"><i l=\"4106\" id=\"1152921510436607008\" s=\"0\" eq=\"1\"/><i l=\"2524\" id=\"1152921510436607009\" s=\"1\" eq=\"1\"/></in></items></inv>")?;
		Ok(())
	}

	fn run_service_mut(&mut self, service: &mut GameObjectServiceMut, game_object: &mut GameObject) -> Res<()> {
		match service {
			GameObjectServiceMut::AddItem(add_item) => {
				let add = AddItemToInventoryClientSync {
					bound: false,
					is_boe: false,
					is_bop: false,
					loot_type_source: LootType::None,
					extra_info: lnv! {},
					obj_template: add_item.lot,
					subkey: 0,
					inv_type: InventoryType::Default,
					item_count: 1,
					items_total: 1,
					new_obj_id: add_item.state.new_spawned_id(),
					flying_loot_posit: Vector3::default(),
					show_flying_loot: true,
					slot_id: 2,
				};
				let gm = game_object.make_sgm(add);
				add_item.conn.send(gm)?;
			},
			_ => {},
		}
		Ok(())
	}

	fn on_game_message(&mut self, msg: &ServerGM, game_object: &mut GameObject, state: &mut State, conn: &mut Connection) -> Res<()> {
		match msg {
			ServerGM::EquipInventory(x) => self.on_equip_inventory(x, game_object, state, conn),
			_ => Ok(()),
		}
	}
}
