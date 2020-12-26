use lu_packets::{
	raknet::client::replica::{ComponentConstruction,
		inventory::{EquippedItemInfo, InventoryConstruction},
	},
};

use super::Component;

pub struct InventoryComponent {

}

impl Component for InventoryComponent {
	fn new() -> Box<dyn Component> {
		Box::new(Self {})
	}

	fn make_construction(&self) -> Box<dyn ComponentConstruction> {
		Box::new(InventoryConstruction {
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
			equipped_model_transforms: Some(vec![].into()),
		})
	}
}
