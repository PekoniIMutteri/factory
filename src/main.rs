mod display;
mod types;
mod input;
mod update;

const MAP_SIZE: types::Vec2 = types::Vec2 {x: 10, y: 10};

fn main() {
    let mut world = types::World::new();
    world.set(types::Vec2::new(7, 2), types::Building::Belt);
    world.set(types::Vec2::new(1, 4), types::Building::Belt);
    world.set(types::Vec2::new(6, 5), types::Building::Belt);
    world.set(types::Vec2::new(6, 6), types::Building::Belt);
    world.set(types::Vec2::new(5, 6), types::Building::Belt);
    world.set(types::Vec2::new(3, 7), types::Building::Belt);
    world.set(types::Vec2::new(3, 9), types::Building::Belt);
    world.set(types::Vec2::new(8, 9), types::Building::Belt);
    display::map(&world);
    loop {
        input::wait_enter();
        update::tick(&mut world);
        display::map(&world);
    }
}
