use mlx::{Mlx, MlxError};
use std::process;

fn julia(x: i32, y: i32, image: &mlx::MlxImage) -> u32 {
    let mut zx = 1.5 * (x as f64 - image.width as f64 / 2.0) / (0.5 * image.width as f64);
    let mut zy = (y as f64 - image.height as f64 / 2.0) / (0.5 * image.height as f64);
    let mut i = 255;
    while zx * zx + zy * zy < 4.0 && i > 1 {
        let xtemp = zx * zx - zy * zy + 0.285;
        zy = 2.0 * zx * zy + 0.01;
        zx = xtemp;
        i -= 1;
    }
    i
}

fn main() {
    let mlx = Mlx::new().unwrap();

    let width = 1080;
    let height = 720;
    let window = mlx.new_window(width, height, "Fractol").unwrap();

    let image = match mlx.new_image(width, height) {
        Ok(img) => img,
        Err(e) => match e {
            MlxError::Any(s) => return println!("{}", s),
            _ => return,
        },
    };

    println!("{}, {}", image.size_line, image.bits_per_pixel);

    window.loop_hook(
        move |_| {
            println!("loop");
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
            // Enter
            } else if keycode == 65293 {
                for y in 0..height {
                    for x in 0..width {
                        let color = julia(x, y, &image);
                        image.pixel_put(x, y, color);
                    }
                }
                mlx.put_image_to_window(&window, &image, 0, 0);
            }
        },
        &(),
    );

    // this will loop forever
    mlx.event_loop();
}
