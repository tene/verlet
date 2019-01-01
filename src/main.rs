use piston_window::*;

use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, Entities, Join, Read, ReadExpect,
    ReadStorage, System, VecStorage, World, WriteStorage,
};
use specs_derive::*;

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

struct TimeStep(f64);
struct PrevTimeStep(f64);

#[derive(Clone, Copy)]
struct Cursor(f64, f64);

struct Verlet;

impl<'a> System<'a> for Verlet {
    type SystemData = (
        ReadExpect<'a, TimeStep>,
        ReadExpect<'a, PrevTimeStep>,
        ReadStorage<'a, Acceleration>,
        ReadStorage<'a, Mass>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PrevPosition>,
    );

    fn run(&mut self, (time, prevtime, force, mass, mut pos, mut prevpos): Self::SystemData) {
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
        Read<'a, Option<Cursor>>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Acceleration>,
    );

    fn run(&mut self, (entities, cursor, pos, mut force): Self::SystemData) {
        /*unimplemented!()*/
    }
}

fn add_particle(world: &mut World, x: f64, y: f64, mass: f64) {
    world
        .create_entity()
        .with(Position { x, y })
        .with(PrevPosition { x, y })
        .with(Mass(mass))
        .build();
    world.maintain();
}

fn build_world<'a, 'b>() -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();

    world.add_resource(PrevTimeStep(0.1));

    let mut dispatcher = DispatcherBuilder::new()
        .with(Verlet, "verlet", &[])
        .with(Gravity, "gravity", &["verlet"])
        .build();

    dispatcher.setup(&mut world.res);

    add_particle(&mut world, 50.0, 50.0, 10.0);

    (world, dispatcher)
}

fn handle_input(world: &mut World, input: Input) {
    match input {
        Input::Button(args) => match (args.state, args.button, args.scancode) {
            (ButtonState::Press, Button::Mouse(MouseButton::Left), None) => {
                let cursor = *world.read_resource::<Option<Cursor>>();
                if let Some(Cursor(x, y)) = cursor {
                    add_particle(world, x, y, 5.0);
                }
            }
            _ => {}
        },
        Input::Move(Motion::MouseCursor(x, y)) => {
            *world.write_resource::<Option<Cursor>>() = Some(Cursor(x, y));
        }
        Input::Cursor(false) => {
            *world.write_resource::<Option<Cursor>>() = None;
        }
        _ => {}
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap();

    let (mut world, mut dispatcher) = build_world();
    while let Some(e) = window.next() {
        match e {
            Event::Input(input) => handle_input(&mut world, input),
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
            Event::Loop(Loop::Update(UpdateArgs { dt })) => {
                world.add_resource(TimeStep(dt));
                dispatcher.dispatch(&mut world.res);
            }
            Event::Loop(Loop::AfterRender(_)) => {}
            Event::Loop(Loop::Idle(IdleArgs { dt: _dt })) => {}
            Event::Custom(_, _) => {}
        }
    }
}
