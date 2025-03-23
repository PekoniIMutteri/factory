use crate::types::{World, Building, Vec2};

pub fn tick(world: &mut World){
    let mut changes = Vec::new();
    for (y, line) in world.map.iter().enumerate() {
        for (x, build) in line.iter().enumerate() {
            match build {
                Building::Belt => changes.push(Vec2::new(x, y)),
                _ => (),
            }
        }
    }
    for change in changes {
        if world.map[0].len() < change.x + 2{
            continue;
        }
        if !world.get(change + Vec2::RIGHT).is_none() {
            continue;
        }
        world.set(change + Vec2::RIGHT, world.get(change));
        world.set(change, Building::None);
    }
}
