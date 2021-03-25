use lu_packets::world::Vector3;
use lu_packets_derive::FromVariants;

#[derive(Debug, FromVariants)]
pub enum GameObjectService<'a> {
	GetPosition(&'a mut GetPosition),
}

#[derive(Debug)]
pub struct GetPosition {
	pub position: Vector3,
}