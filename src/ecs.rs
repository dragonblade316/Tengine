extern crate bevy_ecs as ecs;

struct Tecs {
    world: ecs::world::World
}

unsafe_singleton!(Tecs);

impl Tecs {
    pub fn init() {
        let ins = Self {
            world: ecs::world::World::default()
        };

        Tecs::set_instance(ins);
    }
    
    pub fn resister_entity() {

    }
    pub fn run() {

    }

    //use this only if you have to. hopefully the builtin components and entitys will be enough
    pub fn get_world(&self) -> &ecs::world::World {
        &self.world
    }
    //use this only if you have to. hopefully the builtin components and entitys will be enough
    pub fn get_mut_world(&mut self) -> &mut ecs::world::World {
        &mut self.world
    }
}