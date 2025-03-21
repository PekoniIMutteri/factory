mod display;
mod types;

const MAP_SIZE: types::Vec2 = types::Vec2 {x: 10, y: 10};

fn main() {
    let mut world = types::World::new();
    let new_pos = types::Vec2::new(5, 5);
    world.set(new_pos, types::Building::Belt);
    display::map(&world);
}
