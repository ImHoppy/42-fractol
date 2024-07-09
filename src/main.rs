use mlx::{Mlx, MlxError};
use num_complex::Complex;
use std::process;

const MAX_ITERATIONS: u32 = 110;
const JULIA_CONSTANT: Complex<f32> = Complex::new(-0.9, 0.27015);

fn julia(x: i32, y: i32, image: &mlx::MlxImage) -> u32 {
    let inner_height = image.height as f32;
    let inner_width = image.width as f32;
    let inner_y = y as f32;
    let inner_x = x as f32;

    let mut zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
    let mut zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

    let mut i = MAX_ITERATIONS;

    while zx * zx + zy * zy < 4.0 && i > 1 {
        let tmp = zx * zx - zy * zy + JULIA_CONSTANT.re;
        zy = 2.0 * zx * zy + JULIA_CONSTANT.im;
        zx = tmp;
        i -= 1;
    }

    let r = (i << 3) as u8;
    let g = (i << 5) as u8;
    let b = (i * 4) as u8;
    let color = (r as u32) << 16 | (g as u32) << 8 | b as u32;
    color
}

fn main() {
    let mlx = Mlx::new().unwrap();

    let width = 1080;
    let height = 720;
    let window = mlx.new_window(width, height, "Fractol").unwrap();

    let image = mlx.new_image(width, height).unwrap();

    println!("{}, {}", image.size_line, image.bits_per_pixel);

    mlx.loop_hook(
        move |_| {
            for y in 0..height {
                for x in 0..width {
                    let color = julia(x, y, &image);
                    image.pixel_put(x, y, color);
                }
            }
            mlx.put_image_to_window(&window, &image, 0, 0);
        },
        &(),
    );

    window.key_hook(
        move |keycode, _| {
            // you can also check keycodes using the `xev` command
            println!("{}", keycode);

            // `q`
            if keycode == 113 {
                mlx.destroy_image(&image);
                mlx.destroy_window(&window);
                mlx.destroy();
                process::exit(0);
            }
        },
        &(),
    );

    // this will loop forever
    mlx.event_loop();
}
