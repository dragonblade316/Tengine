use bevy_ecs::{prelude::Component, system::{Query, Resource, Res}, query::{Added, Changed}};

use super::misc::Transform;


#[derive(Component)]
struct Camera {
    relative_position: Transform,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { relative_position: Transform(nalgebra::Matrix4::zeros()), aspect: 0 as f32, fovy: 45.0, znear: 0.1, zfar: 500.0 }
    }
}

#[derive(Resource)]
struct CurrentCam(Camera);

#[derive(Resource)]
struct DisplaySize {
    x: u32,
    y: u32
}

//dont remember why I needed this so ima just leave it
// fn init_cam(query: Query<&Camera, Added<Camera>>) {

// }

fn update_aspect(displaysize: Option<Res<DisplaySize>>,query: Query<&mut Camera> ) {
    if let Some(displaysize) = displaysize {
        query.for_each(|cam| {
            cam.aspect = displaysize.x as f32 / displaysize.y as f32;
        })
    }
}

fn update_cam(query: Query<(&Camera, &Transform), Changed<Transform>>) {

}