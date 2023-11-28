use bevy::prelude::*;

#[derive(Default, Reflect)]
pub struct Cuboid {
    pub displacement: Vec3,
    pub size: Vec3,
}

#[derive(Component, Reflect)]
pub struct IsVisibleCollisionBox;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct IsRigidBody {
    pub cuboids: Vec<Cuboid>,
    pub show_collision_boxes: bool,
}
