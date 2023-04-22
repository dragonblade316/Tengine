use bevy_ecs::prelude::Component;
use nalgebra::Matrix4;

#[derive(Component)]
pub struct Transform(pub Matrix4<f32>);
