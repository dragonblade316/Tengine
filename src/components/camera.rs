use bevy_ecs::{prelude::Component, system::{Query, Resource, Res}, query::{Added, Changed}};

use super::misc::Transform;


#[derive(Component)]
struct Camera {
    relative_position: Transform,
    perspective: nalgebra::Perspective3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            relative_position: Transform::new(nalgebra::Matrix4::zeros()), 
            perspective: nalgebra::Perspective3::new(0.0, 45.0, 0.1, 500.0)}
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

fn update_aspect(displaysize: Option<Res<DisplaySize>>, mut query: Query<&mut Camera> ) {
    if let Some(displaysize) = displaysize {
        
        let mut cam = query.get_single_mut().expect("morn then one camera");
        
        cam.perspective.set_aspect(displaysize.x as f32 / displaysize.y as f32);
        
        // query.for_each_mut(|cam| {
        //     cam.aspect = displaysize.x as f32 / displaysize.y as f32;
        // })
    }
}

fn update_cam(query: Query<(&Camera, &Transform), Changed<Transform>>) {
    //let proj = nalgebra::perspective(nalgebra::Deg(self.fovy), self.aspect, self.znear, self.zfar);
    //nalgebra::Perspective3::
 


    // #[rustfmt::skip]
    // pub const OPENGL_TO_WGPU_MATRIX: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
    //     1.0, 0.0, 0.0, 0.0,
    //     0.0, 1.0, 0.0, 0.0,
    //     0.0, 0.0, 0.5, 0.0,
    //     0.0, 0.0, 0.5, 1.0,
    // );
}