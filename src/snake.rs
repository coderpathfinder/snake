use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};

const SNAKE_INIT_LENGTH: u32 = 2;

struct Snake {
    direction: Direction,
    head: Position,
    tail: LinkedList<Position>,
    updated_tail_pos: bool
}

impl Snake {
    pub fn new(head: Position) -> Self {
        let (x,y) = (head.x, head.y);
        let mut tail = LinkedList::new();

        for i in 1..(SNAKE_INIT_LENGTH+1) {
            tail.push_back(Position{x : x, y : y-i as i32});
        }

        Self {
            direction: Direction::Down,
            head: Position {x, y},
            tail,
            updated_tail_pos: false
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        if self.tail.len() > 0 {
            self.tail.push_front(self.head.clone());
            self.tail.pop_back();
        }

        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1
        }

        if self.head.x >= width as i32 {
            self.head.x = 0;
        }else if self.head.x < 0 {
            self.head.x = width as i32;
        }else if self.head.y >= height as i32 {
            self.head.y = 0;
        }else if self.head.y < 0 {
            self.head.y = height as i32;
        }

        self.updated_tail_pos = true;

    }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        for block in self.tail.iter() {
            draw_block(colors::SNAKE, block, &ctx, g);
        }

        draw_shake_head(colors::SNAKE, &self.head, &ctx, g, &self.direction);
    }

    pub fn set_dir(&mut self, dir:Direction) {
        if dir == self.direction.opposite() || !self.updated_tail_pos {
            return;
        }

        self.direction = dir;
        self.updated_tail_pos = false;
    }

    pub fn get_len(&self) -> u32 {
        (self.tail.len() as u32) - SNAKE_INIT_LENGTH
    }

    pub fn is_over(&self) -> bool {
        for pos in self.tail.iter() {
            if *pos == self.head {
                return true;
            }
        }
        false
    }

    pub fn next_head_pos(&self) -> Position {
        let mut pos = self.head.clone();

        match self.direction {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1
        }

        pos
    }

    pub fn will_over(&self) -> bool {
        let next = self.next_head_pos();

        for pos in self.tail.iter() {
            if *pos == next {
                return true;
            }
        }

        false
    }

    pub fn grow(&mut self) {
        let last = match self.tail.back() {
            Some(pos) => pos.clone(),
            None => self.head.clone()
        };

        self.tail.push_back(last);
    }

    pub fn get_head(&self) -> &Position {
        &self.head
    }
}