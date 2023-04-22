use bevy_ecs::prelude::Component;
use nalgebra::Matrix4;
use wgpu::Texture;

#[macro_use]
mod utils;
mod components;
mod renderer;
mod texture;
mod ecs;
mod bundles;

extern crate nalgebra as math;

struct thing(u32);

pub fn init(width: u32, height: u32) {
    
}

pub fn run() {}

