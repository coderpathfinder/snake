use piston_window::*;
use rand::Rng;
use keyboard::Key;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};
use crate::snake::*;

const FPS: f64 = 10.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn rand_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();
    
    Position {
        x:rng.gen_range(0, width as i32),
        y:rng.gen_range(0, height as i32)
    }
}

pub struct Game {
    size: (u32, u32),
    over: bool,
    paused: bool,
    waiting_time: f64,
    score: u32,
    snake: Snake,
    food: Position
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: (width, height),
            over: false,
            paused: false,
            waiting_time: 0.0,
            score: 0,
            snake: Snake::new(rand_pos(width, height)),
            food: rand_pos(width, height)
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        draw_block(colors::FOOD, &self.food, &ctx, g);
        self.snake.draw(&ctx, g);

        if self.over {
            draw_overlay(colors::OVERLAY, &ctx, g, self.size);
        }
    }

    pub fn update_score(&mut self) -> u32 {
        self.score = (self.snake.get_len() * 10) as u32;
        self.score
    }

    pub fn update(&mut self, data_time: f64) {
        self.waiting_time += data_time;

        if self.waiting_time > fps_in_ms(FPS) && !self.over && !self.paused {
            self.waiting_time = 0.0;

            if !self.snake.is_over() && !self.snake.will_over() {
                self.snake.update(self.size.0,self.size.1);

                if *self.snake.get_head() == self.food {
                    self.snake.grow();
                    //
                    self.food = rand_pos(self.size.0, self.size.1);
                    self.score = self.update_score();
                }
            }else {
                self.over = true;
            }
        }
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        match key {
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::R | Key::P => self.paused = true;
            _ => {}
        }
    }
}