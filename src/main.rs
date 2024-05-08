use mlx::{Mlx, MlxError};
use std::process;

fn main() {
    let mlx = Mlx::new().unwrap();

    let width = 1080;
    let height = 720;
    let window = mlx.new_window(width, height, "Mlx example").unwrap();

    let image = match mlx.new_image(width, height) {
        Ok(img) => img,
        Err(e) => match e {
            MlxError::Any(s) => return println!("{}", s),
            _ => return,
        },
    };

    println!("{}, {}", image.size_line, image.bits_per_pixel);

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
                let x = width / 2;
                let y = height / 2;
                let color = 0xff0000;
                for i in 0..100 {
                    for j in 0..100 {
                        image.pixel_put(x + i, y + j, color);
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
