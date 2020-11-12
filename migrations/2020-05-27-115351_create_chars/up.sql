create table characters (
  id integer not null primary key,
  username text not null,
	name text unique not null,
	torso_color integer not null,
	legs_color integer not null,
	hair_style integer not null,
	hair_color integer not null,
	eyebrow_style integer not null,
	eye_style integer not null,
	mouth_style integer not null,
	world_zone integer not null,
	world_instance integer not null,
	world_clone integer not null
)
