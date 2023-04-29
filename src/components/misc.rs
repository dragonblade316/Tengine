use bevy_ecs::prelude::Component;
use nalgebra::Matrix4;
use wgpu::util::DeviceExt;

#[derive(Component)]
pub struct Transform {
    transform: Matrix4<f32>,
    transform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}
impl Transform {
    pub fn new(matrix: Matrix4<f32>) -> Self {
        let device = &crate::renderer::Renderer::get_instance().device;
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[matrix]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &Self::get_bind_group_layout(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        Self {
            transform: matrix,
            transform_buffer,
            bind_group,
        }
    }

    pub fn update_transform(&self, matrix: Matrix4<f32>) {
        let queue = &crate::renderer::Renderer::get_instance().queue;
        queue.write_buffer(&self.transform_buffer, 0, bytemuck::cast_slice(&[matrix]))
    }

    pub fn get_transform_matrix(&self) -> Matrix4<f32> {
        self.transform
    }

    pub fn get_bind_group_layout() -> wgpu::BindGroupLayout {
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
}
