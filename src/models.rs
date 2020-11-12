//! Database models.
use diesel::{Insertable, Queryable};

use crate::schema::characters;

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
pub struct Character {
	/// Object ID.
	pub id: i32,
	/// Name of the account this character belongs to.
	pub username: String,
	/// Name of the character.
	pub name: String,
	/// Torso color.
	pub torso_color: i32,
	/// Legs color.
	pub legs_color: i32,
	/// Hair style.
	pub hair_style: i32,
	/// Hair color.
	pub hair_color: i32,
	/// Eyebrow style.
	pub eyebrow_style: i32,
	/// Eye style.
	pub eye_style: i32,
	/// Mouth style.
	pub mouth_style: i32,
	/// Zone ID of the world where the character is.
	pub world_zone: i32,
	/// Instance ID of the world where the character is.
	pub world_instance: i32,
	/// Clone ID of the world where the character is.
	pub world_clone: i32,
}
