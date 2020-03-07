#[derive(Debug, Clone, PartialEq)]
pub struct Position{
    pub x: i32,
    pub y: i32
}

// impl Position {
//     //让编译器不警告
//     #[allow(dead_code)]
//     //根据方向移动
//     pub fn move_to_dir(&mut self, dir: Direction){
//         match dir {
//             Direction::Up => self.y -= 1,
//             Direction::Down => self.y += 1,
//             Direction::Left => self.x -= 1,
//             Direction::Right => self.x += 1
//         }
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    //获取相反的方向
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}