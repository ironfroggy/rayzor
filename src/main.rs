extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const A: u32 = 0xff000000;
const R: u32 = 0x00ff0000;
const G: u32 = 0x0000ff00;
const B: u32 = 0x000000ff;

fn color(r: f64, g: f64, b: f64) -> u32 {
    return 0xff000000 | ((256-r as u32) << 16) | ((256-g as u32) << 8) | (256-b as u32);
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let cx = (WIDTH / 2) as i32;
        let cy = (HEIGHT / 2) as i32;
        for y in 0..(HEIGHT as i32) {
            for x in 0..(WIDTH as i32) {
                let d = (((cx-x)*(cx-x) + (cy-y)*(cy-y)) as f64).sqrt();
                let mut c: u32 = 0xff000000; 

                if d < 256.0 {
                    c = color(d, d, d);
                }

                buffer[(WIDTH as i32 * y + x) as usize] = c;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

