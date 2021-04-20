use lu_packets::world::{Lot, Quaternion, Vector3};
use lu_packets::raknet::client::replica::controllable_physics::FrameStats;
use lu_packets_derive::FromVariants;

#[derive(Debug, FromVariants)]
pub enum GameObjectService<'a> {
	GetPosition(&'a mut GetPosition),
	GetRotation(&'a mut GetRotation),
}

#[derive(Debug, Default)]
pub struct GetPosition(pub Vector3);

#[derive(Debug, Default)]
pub struct GetRotation(pub Quaternion);

#[derive(FromVariants)]
#[non_exhaustive]
pub enum GameObjectServiceMut<'a> {
	SetFrameStats(&'a FrameStats),
	AddItem(&'a AddItem),
	SetFaction(&'a SetFaction),
}

pub struct AddItem {
	pub lot: Lot,
}

pub struct SetFaction(pub i32);

