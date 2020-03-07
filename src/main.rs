extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod colors;
mod draw;
mod game;
mod physics;
mod snake;

use draw::block_in_pixels;
use game::Game;

const WINDOW_TITLE: &'static str = "snake";
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

fn main() {
    //创建窗口并设置大小等参数
    let size = [block_in_pixels(WIDTH), block_in_pixels(HEIGHT)];
    let mut window: PistonWindow = WindowSettings::new(
            WINDOW_TITLE,
            size
        )
        .resizable(false)//禁止最大化窗口
        .exit_on_esc(true)//按ESC键退出
        .build()
        .unwrap();

        //查找字体文件，加载字体
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
        let mut glyphs = window.load_font(assets.join("retro-gaming.ttf")).unwrap();

        let mut main: Game = Game::new(WIDTH, HEIGHT);//创建游戏实例
        main.start();//取消暂停状态

        //游戏主循环
        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                //获取事件中的按键
                main.key_down(key);
            }

            window.draw_2d(&event, |ctx, g, device| {
                //设置分数显式的字体和为位置大小等参数
                let transform = ctx.transform.trans(10.0, 20.0);
                let mut s = "score: ".to_string();
                //获取分数
                s.push_str(main.update_score().to_string().as_ref());
                clear(colors::BACKGROUND, g);
                text::Text::new_color(colors::SCORE, 15).draw(
                    s.as_ref(),
                    &mut glyphs,
                    &ctx.draw_state,
                    transform,
                    g
                ).unwrap();

                //绘制
                main.draw(ctx, g);
                glyphs.factory.encoder.flush(device);
            });

            //更新
            event.update( |arg| {
                main.update(arg.dt);
            } );
        }

}