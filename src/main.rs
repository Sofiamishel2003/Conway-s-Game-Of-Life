mod color;
mod framebuffer;
mod bmp;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

fn render(framebuffer: &mut Framebuffer) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(20, 40);
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the framebuffer
        framebuffer.set_background_color(0x333355);
        framebuffer.clear();

        // Render
        render(&mut framebuffer);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.get_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
