mod game_of_life;
mod framebuffer;

use game_of_life::GameOfLife;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

fn render(framebuffer: &mut Framebuffer, game: &GameOfLife) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Draw the cells
    for y in 0..game.height() {
        for x in 0..game.width() {
            if game.is_alive(x, y) {
                framebuffer.set_current_color(0xFFFFFF);
            } else {
                framebuffer.set_current_color(0x000000);
            }
            framebuffer.point(x as isize, y as isize);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100); // Aumentar el delay para hacer la simulación más lenta

    let mut game = GameOfLife::new(framebuffer_width, framebuffer_height);

    // Set initial patterns
    game.set_block(5, 5);
    game.set_block(20, 70);
    game.set_beehive(10, 90);
    game.set_beehive(15, 80);
    game.set_loaf(65, 5);
    game.set_loaf(80, 11);
    game.set_boat(5, 80);
    game.set_boat(20, 28);
    game.set_tub(35, 90);
    game.set_tub(50, 25);
    game.set_blinker(15, 28);
    game.set_blinker(80, 30);
    game.set_toad(5, 35);
    game.set_toad(20, 30);
    game.set_beacon(35, 35);
    game.set_beacon(50, 40);
    game.set_pulsar(20, 50);
    game.set_pulsar(80, 45);
    game.set_pentadecathlon(5, 70);
    game.set_glider(20, 80);
    game.set_lwss(35, 90);
    game.set_mwss(30, 80);
    game.set_hwss(65, 90);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update game state
        game.update();

        // Render game state to framebuffer
        render(&mut framebuffer, &game);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer(), framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
