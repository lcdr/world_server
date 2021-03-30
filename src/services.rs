use lu_packets::world::{Quaternion, Vector3};
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

#[derive(Debug, FromVariants)]
#[non_exhaustive]
pub enum GameObjectServiceMut<'a> {
	SetFrameStats(&'a FrameStats),
}
