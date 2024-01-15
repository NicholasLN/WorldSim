use crate::library::create_world;
use super::game::World;

pub struct State {
    pub tick: u32,
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        Self {
            tick: 0,
            world: World::new(),
        }
    }
    pub fn initiate(&mut self) {
        self.world = World::new();
        let world = create_world();
        // Handle result
        match world {
            Ok(world) => {
                self.world = world;
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}