mod framebuffer;
mod game_of_life;

use std::time::Duration;
use std::thread;
use raylib::prelude::*;
use framebuffer::Framebuffer;
use game_of_life::GameOfLife;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let cell_size = 15;

    let grid_width = window_width / cell_size;
    let grid_height = window_height / cell_size;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);
    framebuffer.set_current_color(Color::BLUE);

    let mut game = GameOfLife::new(grid_width as usize, grid_height as usize);

    while !window.window_should_close() {
        framebuffer.clear();
        game.update();
        game.draw(&mut framebuffer, cell_size as u32);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(100));
    }
}
