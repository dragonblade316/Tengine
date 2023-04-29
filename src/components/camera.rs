use bevy_ecs::{
    prelude::Component,
    query::{Added, Changed},
    system::{Query, Res, Resource},
};
use nalgebra::{Matrix4, Perspective3};

use crate::renderer;

use super::misc::Transform;
use wgpu::util::DeviceExt;

#[derive(Component)]
pub struct Camera {
    pub relative_position: Transform,
    pub perspective: nalgebra::Perspective3<f32>,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Camera {
    //TODO: implment new

    pub fn get_cam_bind_group_layout() -> wgpu::BindGroupLayout {
        let device = &crate::renderer::Renderer::get_instance().device;
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        })
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

impl Default for Camera {
    fn default() -> Self {
        //TODO: most of this needs to be moved to new()
        let device = &crate::renderer::Renderer::get_instance().device;

        let relative_position = Transform::new(nalgebra::Matrix4::zeros());
        let perspective = nalgebra::Perspective3::new(0.0, 45.0, 0.1, 500.0);

        let prog = perspective.as_matrix();
        let view: Matrix4<f32> = nalgebra::Matrix4::zeros();
        #[rustfmt::skip]
        pub const OPENGL_TO_WGPU_MATRIX: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );

        let data: Matrix4<f32> = prog * view * OPENGL_TO_WGPU_MATRIX;

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &Camera::get_cam_bind_group_layout(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            relative_position,
            perspective,
            buffer,
            bind_group,
        }
    }
}

#[derive(Resource)]
struct CurrentCam(Camera);

#[derive(Resource)]
struct DisplaySize {
    x: u32,
    y: u32,
}

//dont remember why I needed this so ima just leave it
// fn init_cam(query: Query<&Camera, Added<Camera>>) {

// }

fn update_aspect(displaysize: Option<Res<DisplaySize>>, mut query: Query<&mut Camera>) {
    if let Some(displaysize) = displaysize {
        let mut cam = query.get_single_mut().expect("morn then one camera");

        cam.perspective
            .set_aspect(displaysize.x as f32 / displaysize.y as f32);

        // query.for_each_mut(|cam| {
        //     cam.aspect = displaysize.x as f32 / displaysize.y as f32;
        // })
    }
}

fn update_cam(query: Query<(&Camera, &Transform), Changed<Transform>>) {
    let (cam, transform) = query.get_single().unwrap();

    let prog = cam.perspective.as_matrix();
    let view: Matrix4<f32> = transform.get_transform_matrix();
    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    let data: Matrix4<f32> = prog * view * OPENGL_TO_WGPU_MATRIX;
    let queue = &crate::renderer::Renderer::get_instance().queue;

    queue.write_buffer(&cam.buffer, 0, bytemuck::cast_slice(&[data]));
}
