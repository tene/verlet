use piston_window::*;

use specs::{
    Builder, Component, DispatcherBuilder, Entities, Join, Read, ReadStorage, System, VecStorage,
    World, WriteStorage,
};
use specs_derive::*;

use std::time::Duration;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct PrevPosition {
    x: f64,
    y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Mass(f64);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Acceleration {
    x: f64,
    y: f64,
}

struct Verlet;

impl<'a> System<'a> for Verlet {
    type SystemData = (
        Read<'a, Duration>,
        ReadStorage<'a, Acceleration>,
        ReadStorage<'a, Mass>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PrevPosition>,
    );

    fn run(&mut self, (time, force, mass, mut pos, mut prevpos): Self::SystemData) {
        /*
        for (force, mass, pos, prevpos) in (&force, &mass, &mut pos, &mut prevpos).join() {
            unimplemented!()
        }
        unimplemented!()*/
    }
}

struct Gravity;

impl<'a> System<'a> for Gravity {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Acceleration>,
    );

    fn run(&mut self, (entities, pos, mut force): Self::SystemData) {
        /*unimplemented!()*/
    }
}

fn build_world() -> World {
    let mut world = World::new();

    /*world.add_resource(Duration::from_secs(0));

    world.register::<Position>();
    world.register::<PrevPosition>();
    world.register::<Mass>();
    world.register::<Force>();*/

    let mut dispatcher = DispatcherBuilder::new()
        .with(Verlet, "verlet", &[])
        .with(Gravity, "gravity", &["verlet"])
        .build();

    dispatcher.setup(&mut world.res);

    world
        .create_entity()
        .with(Position { x: 51.0, y: 51.0 })
        .with(PrevPosition { x: 50.9, y: 50.9 })
        .with(Mass(10.0))
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
    world
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap();

    let mut world = build_world();
    while let Some(e) = window.next() {
        match e {
            Event::Input(_input) => {}
            Event::Loop(Loop::Render(_args)) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 0.5, 1.0], g);
                    let (positions, masses): (ReadStorage<Position>, ReadStorage<Mass>) =
                        world.system_data();
                    for (pos, mass) in (&positions, &masses).join() {
                        ellipse(
                            [0.6, 0.5, 1.0, 1.0],
                            ellipse::circle(pos.x, pos.y, mass.0),
                            c.transform,
                            g,
                        );
                    }
                });
            }
            Event::Loop(Loop::Update(UpdateArgs { dt: _dt })) => {
                world.maintain();
            }
            Event::Loop(Loop::AfterRender(_)) => {}
            Event::Loop(Loop::Idle(IdleArgs { dt: _dt })) => {}
            Event::Custom(_, _) => {}
        }
    }
}
