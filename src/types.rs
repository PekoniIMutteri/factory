use crate::MAP_SIZE;

pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}
impl Vec2 {
    pub fn new(x: usize, y: usize) -> Vec2 {
        Vec2 {x, y,}
    }
}

#[derive(Clone, Copy)]
pub enum Building {
    None,
    Belt,
}
impl Building {
    pub fn to_char(&self) -> char {
        match self {
            Building::None => ' ',
            Building::Belt => '#',
        }
    }
}

pub struct World {
    pub map: [[Building; MAP_SIZE.x]; MAP_SIZE.y],
}
impl World {
    pub fn new() -> World {
        World {
            map: [[Building::None; MAP_SIZE.x]; MAP_SIZE.y],
        }
    }
    pub fn set(&mut self, position: Vec2, building: Building) {
        self.map[position.y][position.x] = building;
    }
}
