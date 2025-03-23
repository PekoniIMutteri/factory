use std::io::Write;

use crate::types::{Vec2, World};

fn _go_to_vec(position: Vec2) {
    print!("\x1B[{};{}H", position.y, position.x);
}

fn go_to(x: usize, y: usize) {
    print!("\x1B[{};{}H", y, x);
}

fn clear() {
    print!("\x1B[2J")
}

pub fn map(world: &World) {
    clear();
    go_to(1, 1);
    for (height, line) in world.map.iter().enumerate() {
        go_to(1, height);
        for building in line {
            print!("{}", building.to_char());
        }
    }
    let _ = std::io::stdout().flush();
}
