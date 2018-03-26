extern crate specs;
use specs::{Component, ReadStorage, System, VecStorage, World, RunNow};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;
    type SystemData1 = ReadStorage<'a, Velocity>;

    fn run(&mut self, position: Self::SystemData, velocity: Self::SystemData1) {
        use specs::Join;

        for position in position.join() {
            println!("Position: {:?}", &position);
        }

        for velocity in velocity.join() {
            println!("Velocity: {:?}", &velocity);
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world.create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Velocity { x: 3.2, y: 4.0 })
        .build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world.res);
}