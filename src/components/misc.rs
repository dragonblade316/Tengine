use bevy_ecs::prelude::Component;
use nalgebra::Matrix4;

#[derive(Component)]
struct Transform {
    transform: Matrix4<f32>
}
