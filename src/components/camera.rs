use bevy_ecs::prelude::Component;


#[derive(Component)]
struct camrea {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}
