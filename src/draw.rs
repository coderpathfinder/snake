use crate::colors;
use crate::physics::{Direction, Position};
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 20.0;

//绘制正方形
pub fn draw_block(c: Color, pos: &Position, ctx: &Context, g: &mut G2d) {
    rectangle(
        c,
        [
            (pos.x as f64) * BLOCK_SIZE,
            (pos.y as f64) * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE
        ],
        ctx.transform,
        g
    );
}

//绘制蛇头，重要的是蛇的眼睛要随移动的方向改变
pub fn draw_shake_head(c: Color, pos: &Position, ctx: &Context, g: &mut G2d, dir: &Direction) {
    draw_block(c, pos, ctx, g);

    fn draw_eye(ctx: &Context, g: &mut G2d, x: f64, y: f64) {
        rectangle(
            colors::EYE,
            [x, y, 4.0, 4.0],
            ctx.transform,
            g
        );
    }

    let (x,y) = (
        block_in_pixels(pos.x as u32) as f64,
        block_in_pixels(pos.y as u32) as f64
    );

    let block = block_in_pixels(1) as f64;

    match dir {
        Direction::Up => {
            draw_eye(ctx, g, x+4.0, y+4.0);
            draw_eye(ctx, g, x+block-8.0, y+4.0);
        }
        Direction::Down => {
            draw_eye(ctx, g, x+4.0, y+block-8.0);
            draw_eye(ctx, g, x+block-8.0, y+block-8.0);
        }
        Direction::Left => {
            draw_eye(ctx, g, x+4.0, y+4.0);
            draw_eye(ctx, g, x+4.0, y+block-8.0);
        }
        Direction::Right => {
            draw_eye(ctx, g, x+block-8.0, y+4.0);
            draw_eye(ctx, g, x+block-8.0, y+block-8.0);
        }
    }
}

pub fn block_in_pixels(n: u32) -> u32 {
    n * BLOCK_SIZE as u32
}

//绘制游戏结束画面
pub fn draw_overlay(c: Color, ctx: &Context, g: &mut G2d, size: (u32,u32)) {
    rectangle(
        c,
        [
            0.0,
            0.0,
            block_in_pixels(size.0) as f64,
            block_in_pixels(size.1) as f64
        ],
        ctx.transform,
        g
    );
}