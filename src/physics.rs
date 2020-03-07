#[derive(Debug, Clone, PartialEq)]
pub struct Position{
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn move_to_dir(&mut self, dir: Direction){
        match dir {

        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}