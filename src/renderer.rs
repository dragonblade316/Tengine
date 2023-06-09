use anyhow::Ok;
use hash_map::HashMap;
use std::collections::hash_map;
use std::ops::RemAssign;

use bevy_ecs::system::Query;
use wgpu::{RenderPass, RenderPipeline, SurfaceTexture};
use winit::{event_loop::EventLoop, window::Window};

use crate::components::camera::{self, Camera};
use crate::components::light::Light;
use crate::components::misc::Transform;

pub struct Renderer {
    window: Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    render_pipelines: hash_map::HashMap<&'static str, wgpu::RenderPipeline>,
    depth_texture: crate::texture::Texture,
}

unsafe_singleton!(Renderer);

impl Renderer {
    async fn new(event_loop: &EventLoop<()>) -> Self {
        let window = Window::new(event_loop).expect("Unable to open window");

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let depth_texture =
            crate::texture::Texture::create_depth_texture(&device, &config, "depth texture");

        let mut render_pipelines = hash_map::HashMap::new();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            //TODO: this will need to be changed to std::path
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "..\\res\\shaders\\shader.wgsl"
            ))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[crate::components::model::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),

            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        render_pipelines.insert("standard", render_pipeline);

        Self {
            window,
            surface,
            size,
            device,
            queue,
            config,
            render_pipelines,
            depth_texture,
        }
    }

    async fn init(event_loop: &EventLoop<()>) {
        let instance = Renderer::new(event_loop).await;
        Renderer::set_instance(instance);
    }

    //TODO: this is bad. rewrite it
    pub fn render(
        &mut self,
        models: Query<(&Model, &Transform)>,
        lights: Query<&Light, &Transform>,
        cam: Query<&Camera>,
    ) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pipeline = self.render_pipelines.get("standard").unwrap();

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],

            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(3, cam.single().get_bind_group(), &[]);

        models.for_each(|m| {
            let (model, transform) = m;
            render_pass.render_model(transform, model);
        });

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

struct TRenderPass<'a> {
    output: SurfaceTexture,
    encoder: wgpu::CommandEncoder,
    pub render_pass: Option<wgpu::RenderPass<'a>>,
}

impl<'a> TRenderPass<'a> {
    pub fn display(mut self) {
        let queue = &Renderer::get_instance().queue;

        //renderpass is dropped to regain use of encoder
        self.render_pass = None;

        queue.submit(std::iter::once(self.encoder.finish()));
        self.output.present();

        //this will ensure the user does not try to use this after the renderpass is deleted
    }
}

use crate::components::model::{self, Material, Mesh, Model};
use crate::ecs;

pub trait RenderMesh<'a> {
    fn render_model(&mut self, position: &'a Transform, model: &'a Model);
    fn render_mesh(&mut self, position: &'a Transform, mesh: &'a Mesh, mat: &'a Material);
}

impl<'a, 'b> RenderMesh<'a> for wgpu::RenderPass<'b>
where
    'a: 'b,
{
    //TODO: I will need to rewrite this to support animations later
    fn render_model(&mut self, position: &'a Transform, model: &'a Model) {
        for i in &model.meshes {
            self.render_mesh(position, &i, &model.materials[i.material_index as usize])
        }
    }
    fn render_mesh(&mut self, position: &'a Transform, mesh: &'a Mesh, mat: &'a Material) {
        //bind group zero will always be reserved for mats. bind group 1 will be for the position. bind group 2 will be for the camera which is set in the camera file
        self.set_bind_group(0, &mat.bind_group, &[]);
        self.set_bind_group(1, &position.bind_group, &[]);

        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        self.draw_indexed(0..mesh.num_elements, 0, 0..1)
    }
}
