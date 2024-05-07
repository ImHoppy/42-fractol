#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
//! This library constains rust bindings for the [linux minilibx](https://github.com/42Paris/minilibx-linux), a simple X11 GUI toolkit for beginners.
//!
//! The bindings should work as expected, if it's not the case, feel free to open an issue [here](https://github.com/kyazdani42/minilibx-rs/issues).

//! # Dependencies
//! - libX11
//! - libXext
//! - libmlx (minilibx)

/*! # Example
```rust
extern crate minilibx;

use std::process;
use minilibx::{Mlx, MlxError};

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
                process::exit(0);
            // Enter
            } else if keycode == 97 {
                let x = width / 2;
                let y = height / 2;
                let color = 0xffffff;
                for i in 0..50 {
                    mlx.pixel_put(&window, x + i, y + i, color);
                }
            }
        },
        &(),
    );

    // this will loop forever
    mlx.event_loop();
}
```
*/

use std::ffi::c_void;

mod ffi;

pub use ffi::MlxError;

/// Api method holder.
#[derive(Clone, Copy)]
pub struct Mlx {
    mlx_ptr: *mut c_void,
}

impl Mlx {
    /// Creates a new Mlx instance.
    ///
    /// Calls the `mlx_init` C method under the hood.
    ///
    /// Usage:
    ///```
    ///let mlx = Mlx::new().unwrap();
    ///```
    pub fn new() -> Result<Self, MlxError> {
        Ok(Self {
            mlx_ptr: ffi::init()?,
        })
    }

    /// Creates a new [window](struct.MlxWindow.html) instance.
    ///
    /// Usage:
    ///```
    /// let image = mlx.new_window(1920, 1080, "mlx-example").unwrap();
    ///```
    pub fn new_window(&self, size_x: i32, size_y: i32, title: &str) -> Result<MlxWindow, MlxError> {
        Ok(MlxWindow {
            win_ptr: ffi::new_window(self.mlx_ptr, size_x, size_y, title)?,
        })
    }

    /// Clears the window with black.
    pub fn clear_window(&self, window: &MlxWindow) {
        ffi::clear_window(self.mlx_ptr, window.win_ptr);
    }

    /// Destroys the window. This function also drops the window object.
    pub fn destroy_window(&self, window: &MlxWindow) {
        ffi::destroy_window(self.mlx_ptr, window.win_ptr);
        drop(window);
    }

    /// Get the actual screen size.
    pub fn get_screen_size(&self) -> (i32, i32) {
        ffi::get_screen_size(self.mlx_ptr)
    }

    /// Put a pixel on the screen
    ///
    /// You should encode the color as RGB on the three last bytes of the int.
    ///
    /// 0x00|ff(R)|ff(G)|ff(B)
    ///
    /// Usage:
    ///```
    /// let x = 200;
    /// let y = 300;
    /// let color = 0x0000ff; // blue
    /// mlx.pixel_put(&window, x, y, color);
    ///```
    pub fn pixel_put(&self, window: &MlxWindow, x: i32, y: i32, color: i32) {
        ffi::pixel_put(self.mlx_ptr, window.win_ptr, x, y, color);
    }

    /// Writes a string on the screen
    ///
    /// Color is encoded in rgb as well.
    /// Usage:
    ///```
    /// let x = 200;
    /// let y = 300;
    /// let color = 0xff0000; // red
    /// mlx.string_put(&window, x, y, color, "Hello World");
    ///```
    pub fn string_put(
        &self,
        window: &MlxWindow,
        x: i32,
        y: i32,
        color: i32,
        s: &str,
    ) -> Result<(), MlxError> {
        ffi::string_put(self.mlx_ptr, window.win_ptr, x, y, color, s)
    }

    /// Creates a new [image](struct.MlxImage.html).
    pub fn new_image(&self, width: i32, height: i32) -> Result<MlxImage, MlxError> {
        let ptr = ffi::new_image(self.mlx_ptr, width, height)?;
        let image = MlxImage::new(ptr, width, height)?;
        Ok(image)
    }

    /// Creates a new [image](struct.MlxImage.html) from [xpm](https://en.wikipedia.org/wiki/X_PixMap) data.
    ///
    /// Note that the minilibx does not use the standard Xpm library. You may not be able to read all types of xpm images.
    ///
    /// It however handles transparency.
    pub fn xpm_to_image(&self, xpm_data: Vec<String>) -> Result<MlxImage, MlxError> {
        let data = ffi::xpm_to_image(self.mlx_ptr, xpm_data)?;
        let image = MlxImage::new(data.ptr, data.width, data.height)?;
        Ok(image)
    }


    /// Creates a new [image](struct.MlxImage.html) from an [xpm](https://en.wikipedia.org/wiki/X_PixMap) file.
    pub fn xpm_file_to_image(&self, filename: &str) -> Result<MlxImage, MlxError> {
        let data = ffi::xpm_file_to_image(self.mlx_ptr, filename)?;
        let image = MlxImage::new(data.ptr, data.width, data.height)?;
        Ok(image)
    }

    /// Destroy the image. Also drops the image instance.
    pub fn destroy_image(&self, image: &MlxImage) {
        ffi::destroy_image(self.mlx_ptr, image.img_ptr);
        drop(image);
    }

    /// Draws an image to the window
    ///
    /// Usage:
    ///```
    /// let x = 200;
    /// let y = 200;
    /// mlx.put_image_to_window(&window, &image, x, y);
    ///```
    pub fn put_image_to_window(&self, window: &MlxWindow, image: &MlxImage, x: i32, y: i32) {
        ffi::put_image_to_window(self.mlx_ptr, window.win_ptr, image.img_ptr, x, y);
    }

    /// Transforms an RGB color parameter into a u32 value.
    ///
    /// This returns a bits_per_pixel value of the rgb value.
    ///
    /// You can use this to write into an [image](struct.MlxImage.html)
    pub fn get_color_value(&self, color: i32) -> u32 {
        ffi::get_color_value(self.mlx_ptr, color)
    }

    /// Enables key autorepeat when pressing a key
    pub fn do_key_autorepeaton(&self) {
        ffi::do_key_autorepeaton(self.mlx_ptr)
    }

    /// Disables key autorepeat when pressing a key
    pub fn do_key_autorepeatoff(&self) {
        ffi::do_key_autorepeatoff(self.mlx_ptr)
    }

    /// Moves the mouse cursor
    pub fn mouse_move(&self, window: &MlxWindow, x: i32, y: i32) {
        ffi::mouse_move(self.mlx_ptr, window.win_ptr, x, y);
    }

    /// Shows the mouse cursor
    pub fn mouse_show(&self, window: &MlxWindow) {
        ffi::mouse_show(self.mlx_ptr, window.win_ptr);
    }

    /// Hides the mouse cursor
    pub fn mouse_hide(&self, window: &MlxWindow) {
        ffi::mouse_hide(self.mlx_ptr, window.win_ptr);
    }

    /// Run the event loop.
    ///
    /// This is running an infinite loop which launches [hooks](struct.MlxWindow.html) when receiving events.
    pub fn event_loop(&self) {
        ffi::event_loop(self.mlx_ptr);
    }
}

/// Hook api holder. Needed in most [Mlx](struct.Mlx.html) methods.
///
/// With hooks, you can provide closures that will run when an event occurs.
///
/// The [mlx.event_loop](struct.Mlx.html#method.event_loop) method should run for these hooks to be executed.
#[derive(Clone, Copy)]
pub struct MlxWindow {
    win_ptr: *mut c_void,
}

impl MlxWindow {
    /// Hook running whenever a mouse event is received.
    ///
    /// F should be a closure taking 4 arguments: the buttons, x, y and the data you provide as last argument of the mouse_hook call.
    ///
    /// Usage:
    /// ```
    /// let arg = (2, 3);
    /// window.mouse_hook(|buttons, x, y, args| {
    ///     println!("{} {}, {}, ({}, {})", buttons, x, y, args.0, args.1);
    /// }, &arg);
    /// ```
    pub fn mouse_hook<F, Args>(&self, mut cb: F, args: &'static Args)
    where
        F: FnMut(i32, i32, i32, &'static Args) + 'static,
    {
        ffi::mouse_hook(self.win_ptr, move |buttons: i32, x: i32, y: i32| {
            cb(buttons, x, y, args);
        });
    }

    /// Hook running whenever a key event is received.
    ///
    /// F should be a closure taking 2 arguments: the keycode and the data you provide as last argument of the mouse_hook call.
    ///
    /// Usage:
    /// ```
    /// let arg = (2, 3);
    /// window.key_hook(|keycode, args| {
    ///     println!("{}, ({}, {})", keycode, args.0, args.1);
    /// }, &arg);
    /// ```
    pub fn key_hook<F, Args>(&self, mut cb: F, args: &'static Args)
    where
        F: FnMut(i32, &'static Args) + 'static,
    {
        ffi::key_hook(self.win_ptr, move |keycode| {
            cb(keycode, args);
        });
    }

    /// Hook running whenever an 'expose' event is received.
    ///
    /// F should be a closure taking the data you pass as an argument.
    ///
    /// Usage:
    /// ```
    /// let arg = (2, 3);
    /// window.expose_hook(|args| {
    ///     println!("({}, {})", args.0, args.1);
    /// }, &arg);
    /// ```
    ///
    /// The expose hook is running whenever the window or part of it should be redrawn.
    pub fn expose_hook<F, Args>(&self, mut cb: F, args: &'static Args)
    where
        F: FnMut(&'static Args) + 'static,
    {
        ffi::expose_hook(self.win_ptr, move || {
            cb(args);
        });
    }

    /// Hook running when no event occurs.
    ///
    /// F should be a closure taking the data you pass as an argument.
    ///
    /// Usage:
    /// ```
    /// let arg = (2, 3);
    /// window.loop_hook(|args| {
    ///     println!("({}, {})", args.0, args.1);
    /// }, &arg);
    /// ```
    pub fn loop_hook<F, Args>(&self, mut cb: F, args: &'static Args)
    where
        F: FnMut(&'static Args) + 'static,
    {
        ffi::loop_hook(self.win_ptr, move || {
            cb(args);
        });
    }

    /// Hook running whenever the event you specify occurs.
    ///
    /// F should be a closure taking the data you pass as an argument.
    ///
    /// Usage:
    /// ```
    /// let arg = (2, 3);
    /// let x_event = 2; // keypress
    /// let x_mask = 0; // no mask
    /// window.hook(x_event, x_mask, |args| {
    ///     println!("({}, {})", args.0, args.1);
    /// }, &arg);
    /// ```
    ///
    /// You can find informations on x events in `/usr/include/X11/X.h` around line 180 and x event masks around line 150.
    pub fn hook<F, Args>(&self, x_event: i32, x_mask: i32, mut cb: F, args: &'static Args)
    where
        F: FnMut(&'static Args) + 'static,
    {
        ffi::hook(self.win_ptr, x_event, x_mask, move || {
            cb(args);
        });
    }
}

/// Enum describing the [endianness](https://en.wikipedia.org/wiki/Endianness) of some data.
#[derive(Clone, Copy, Debug)]
pub enum Endian {
    /// little endian variant.
    Little = 0,
    /// big endian variant.
    Big = 1,
}

/// Image data placeholder. Can be used to draw image onto the screen.
#[derive(Clone, Copy)]
pub struct MlxImage {
    img_ptr: *mut c_void,
    /// width of the image
    pub width: i32,
    /// height of the image
    pub height: i32,
    area_start: *mut i8,
    /// number of bits needed to represent a pixel color ([depth](https://en.wikipedia.org/wiki/Color_depth) of the image)
    pub bits_per_pixel: i32,
    /// number of bytes used to store one line of the image in memory.
    ///
    /// needed to move from one line to another in the image.
    pub size_line: i32,
    /// the endianness of the image
    pub endian: Endian,
}

impl MlxImage {
    fn new(img_ptr: *mut c_void, width: i32, height: i32) -> Result<Self, MlxError> {
        let data = ffi::get_data_addr(img_ptr)?;
        Ok(Self {
            img_ptr,
            width,
            height,
            area_start: data.area,
            bits_per_pixel: data.bits_per_pixel,
            size_line: data.size_line,
            endian: if data.endian == 1 {
                Endian::Big
            } else {
                Endian::Little
            },
        })
    }

    /// Writes to the image from offset of the beginning of the area where the image is stored.
    ///
    /// The first bits_per_pixel bits represent the color of the first pixel in the first line of the image.
    ///
    /// The second bits_per_pixel bits represent the second pixel of the first line, and so on.
    ///
    /// This function could cause a segmentation fault if your offset is wrong, be careful !
    pub fn write_to(&self, offset: i32, value: u8) {
        unsafe {
            *self.area_start.offset(offset as isize) = value as i8;
        }
    }
}
