use lu_packets::{
	raknet::client::replica::inventory::{EquippedItemInfo, InventoryConstruction, InventoryProtocol, InventorySerialization},
	world::LuNameValue,
};

use super::InternalComponent;

pub struct InventoryComponent {

}

impl InternalComponent for InventoryComponent {
	type ComponentProtocol = InventoryProtocol;

	fn new(_config: &LuNameValue) -> Self {
		Self {}
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
}
