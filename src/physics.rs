#[derive(Debug, Clone, PartialEq)]
pub struct Position{
    pub x: i32,
    pub y: i32
}

impl Position {
    #[warn(dead_code)]]
    pub fn move_to_dir(&mut self, dir: Direction){
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1
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