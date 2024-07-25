mod color;
mod framebuffer;
mod bmp;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::time::Instant;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut x = 1 as isize;
    let mut speed = 1 as isize;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Prepare variables for rendering
        if x as usize == framebuffer_width {
            speed = -1;
        }
        if x == 0 {
            speed = 1;
        }
        x += speed;

        // Clear the framebuffer
        framebuffer.set_background_color(0x333355);
        framebuffer.clear();

        // Draw some points
        framebuffer.set_current_color(0xFFDDDD);
        framebuffer.point(x, 300);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.get_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();

        // Take screenshot if the user presses 'S'
        if window.is_key_down(Key::S) {
            let timestamp = Instant::now().elapsed().as_secs();
            let file_path = format!("screenshot_{}.bmp", timestamp);
            framebuffer.render_buffer(&file_path).unwrap();
            println!("Screenshot saved to {}", file_path);
        }

        std::thread::sleep(frame_delay);
    }
}
