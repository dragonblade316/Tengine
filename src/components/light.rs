use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Light {
    intensity: f32,
    color: [f32; 3],
}
