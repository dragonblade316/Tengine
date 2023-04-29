use bevy_ecs::prelude::Component;
use nalgebra::Matrix4;
use wgpu::Texture;

#[macro_use]
mod utils;
mod bundles;
mod components;
mod ecs;
mod renderer;
mod texture;

extern crate nalgebra as math;

//TODO: add comments, add user methods, add tests,

pub fn init(name: &'static str, width: u32, height: u32) {
    ecs::Tecs::init();
    //the ecs must be running before the renderer as some of the rendering code depends on it
}
pub fn run() {}
