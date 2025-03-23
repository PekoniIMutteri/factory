use crate::MAP_SIZE;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}
impl Vec2 {
    pub fn new(x: usize, y: usize) -> Vec2 {
        Vec2 {x, y,}
    }
    pub const RIGHT: Self = Vec2 {
        x: 1,
        y: 0,
    };
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
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
    pub fn is_none(&self) -> bool {
        match self {
            Building::None => true,
            _ => false,
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
    pub fn get(&self, position: Vec2) -> Building {
        self.map[position.y][position.x]
    }
}
