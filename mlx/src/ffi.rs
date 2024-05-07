use std::ffi::{c_void, CString};
use std::os::raw::c_char;

fn create_c_str(s: &str) -> Result<CString, MlxError> {
    match CString::new(s) {
        Ok(s) => Ok(s),
        Err(_) => Err(MlxError::Any(format!("Error creating string {}", s))),
    }
}

/// Enum for detecting errors in the minilibx.
#[derive(Debug)]
pub enum MlxError {
    /// Error that could happen when mlx_init returns a null pointer
    Init,
    /// Error that could happen when mlx_new_window returns a null pointer
    Window,
    /// Error that might happen in all other places. It contains the information about the error.
    Any(String),
}

pub fn init() -> Result<*mut c_void, MlxError> {
    extern "C" {
        pub fn mlx_init() -> *mut c_void;
    }

    unsafe {
        let ptr = mlx_init();
        if ptr.is_null() {
            Err(MlxError::Init)
        } else {
            Ok(ptr)
        }
    }
}

pub fn destroy(mlx_ptr: *mut c_void) {
    extern "C" {
        pub fn mlx_destroy_display(mlx_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_destroy_display(mlx_ptr);
        libc::free(mlx_ptr);
    }
}

pub fn new_window(
    mlx_ptr: *mut c_void,
    size_x: i32,
    size_y: i32,
    title: &str,
) -> Result<*mut c_void, MlxError> {
    extern "C" {
        fn mlx_new_window(
            mlx_ptr: *mut c_void,
            size_x: i32,
            size_y: i32,
            title: *const c_char,
        ) -> *mut c_void;
    }

    let title = create_c_str(title)?;

    unsafe {
        let ptr = mlx_new_window(mlx_ptr, size_x, size_y, title.as_ptr());
        if ptr.is_null() {
            Err(MlxError::Window)
        } else {
            Ok(ptr)
        }
    }
}

pub fn clear_window(mlx_ptr: *mut c_void, win_ptr: *mut c_void) {
    extern "C" {
        fn mlx_clear_window(mlx_ptr: *mut c_void, win_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_clear_window(mlx_ptr, win_ptr);
    }
}

pub fn destroy_window(mlx_ptr: *mut c_void, win_ptr: *mut c_void) {
    extern "C" {
        fn mlx_destroy_window(mlx_ptr: *mut c_void, win_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_destroy_window(mlx_ptr, win_ptr);
    }
}

pub fn get_screen_size(mlx_ptr: *mut c_void) -> (i32, i32) {
    extern "C" {
        fn mlx_get_screen_size(mlx_ptr: *mut c_void, sizex: &mut i32, sizey: &mut i32) -> i32;
    }

    let mut sizex = 0;
    let mut sizey = 0;
    unsafe {
        mlx_get_screen_size(mlx_ptr, &mut sizex, &mut sizey);
    }
    (sizex, sizey)
}

pub fn pixel_put(mlx_ptr: *mut c_void, win_ptr: *mut c_void, x: i32, y: i32, color: i32) {
    extern "C" {
        fn mlx_pixel_put(
            mlx_ptr: *mut c_void,
            win_ptr: *mut c_void,
            x: i32,
            y: i32,
            color: i32,
        ) -> i32;
    }

    unsafe {
        mlx_pixel_put(mlx_ptr, win_ptr, x, y, color);
    }
}

pub fn string_put(
    mlx_ptr: *mut c_void,
    win_ptr: *mut c_void,
    x: i32,
    y: i32,
    color: i32,
    s: &str,
) -> Result<(), MlxError> {
    extern "C" {
        fn mlx_string_put(
            mlx_ptr: *mut c_void,
            win_ptr: *mut c_void,
            x: i32,
            y: i32,
            color: i32,
            string: *const c_char,
        ) -> i32;
    }

    let s = create_c_str(s)?;
    unsafe {
        mlx_string_put(mlx_ptr, win_ptr, x, y, color, s.as_ptr());
    }

    Ok(())
}

pub fn new_image(mlx_ptr: *mut c_void, width: i32, height: i32) -> Result<*mut c_void, MlxError> {
    extern "C" {
        fn mlx_new_image(mlx_ptr: *mut c_void, width: i32, height: i32) -> *mut c_void;
    }

    unsafe {
        let ptr = mlx_new_image(mlx_ptr, width, height);
        if ptr.is_null() {
            Err(MlxError::Any(format!(
                "Error creating image with {} width, {} height.",
                width, height
            )))
        } else {
            Ok(ptr)
        }
    }
}

pub struct XpmImage {
    pub ptr: *mut c_void,
    pub width: i32,
    pub height: i32,
}

pub fn xpm_to_image(mlx_ptr: *mut c_void, xpm_data: Vec<String>) -> Result<XpmImage, MlxError> {
    extern "C" {
        fn mlx_xpm_to_image(
            mlx_ptr: *mut c_void,
            xpm_data: *const *const c_char,
            width: &mut i32,
            height: &mut i32,
        ) -> *mut c_void;
    }

    let mut width = 0;
    let mut height = 0;
    unsafe {
        let ptr = mlx_xpm_to_image(
            mlx_ptr,
            xpm_data.as_ptr() as *const *const c_char,
            &mut width,
            &mut height,
        );
        if ptr.is_null() {
            Err(MlxError::Any(format!("Error creating xpm image.",)))
        } else {
            Ok(XpmImage { ptr, width, height })
        }
    }
}

pub fn xpm_file_to_image(mlx_ptr: *mut c_void, filename: &str) -> Result<XpmImage, MlxError> {
    extern "C" {
        fn mlx_xpm_file_to_image(
            mlx_ptr: *mut c_void,
            filename: *const c_char,
            width: &mut i32,
            height: &mut i32,
        ) -> *mut c_void;
    }

    let mut width = 0;
    let mut height = 0;
    let filename = create_c_str(filename)?;
    unsafe {
        let ptr = mlx_xpm_file_to_image(mlx_ptr, filename.as_ptr(), &mut width, &mut height);
        if ptr.is_null() {
            Err(MlxError::Any(format!("Error creating xpm image.",)))
        } else {
            Ok(XpmImage { ptr, width, height })
        }
    }
}

pub fn destroy_image(mlx_ptr: *mut c_void, img_ptr: *mut c_void) {
    extern "C" {
        fn mlx_destroy_image(mlx_ptr: *mut c_void, img_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_destroy_image(mlx_ptr, img_ptr);
    }
}

pub struct AddrData {
    pub area: *mut c_char,
    pub bits_per_pixel: i32,
    pub size_line: i32,
    pub endian: i32,
}

pub fn get_data_addr(img_ptr: *mut c_void) -> Result<AddrData, MlxError> {
    extern "C" {
        fn mlx_get_data_addr(
            img_ptr: *mut c_void,
            bits_per_pixel: &mut i32,
            size_line: &mut i32,
            endian: &mut i32,
        ) -> *mut c_char;
    }

    let area;
    let mut bits_per_pixel = 0;
    let mut size_line = 0;
    let mut endian = 0;
    unsafe {
        area = mlx_get_data_addr(img_ptr, &mut bits_per_pixel, &mut size_line, &mut endian);
        if area.is_null() {
            Err(MlxError::Any(format!(
                "Error when trying to access image data"
            )))
        } else {
            Ok(AddrData {
                area,
                bits_per_pixel,
                size_line,
                endian,
            })
        }
    }
}

pub fn put_image_to_window(
    mlx_ptr: *mut c_void,
    win_ptr: *mut c_void,
    img_ptr: *mut c_void,
    x: i32,
    y: i32,
) {
    extern "C" {
        fn mlx_put_image_to_window(
            mlx_ptr: *mut c_void,
            win_ptr: *mut c_void,
            img_ptr: *mut c_void,
            x: i32,
            y: i32,
        ) -> i32;
    }

    unsafe {
        mlx_put_image_to_window(mlx_ptr, win_ptr, img_ptr, x, y);
    }
}

pub fn get_color_value(mlx_ptr: *mut c_void, color: i32) -> u32 {
    extern "C" {
        fn mlx_get_color_value(mlx_ptr: *mut c_void, color: i32) -> u32;
    }

    unsafe { mlx_get_color_value(mlx_ptr, color) }
}

pub fn do_key_autorepeatoff(mlx_ptr: *mut c_void) {
    extern "C" {
        fn mlx_do_key_autorepeatoff(mlx_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_do_key_autorepeatoff(mlx_ptr);
    }
}

pub fn do_key_autorepeaton(mlx_ptr: *mut c_void) {
    extern "C" {
        fn mlx_do_key_autorepeaton(mlx_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_do_key_autorepeaton(mlx_ptr);
    }
}

pub fn mouse_move(mlx_ptr: *mut c_void, win_ptr: *mut c_void, x: i32, y: i32) {
    extern "C" {
        fn mlx_mouse_move(mlx_ptr: *mut c_void, win_ptr: *mut c_void, x: i32, y: i32) -> i32;
    }

    unsafe {
        mlx_mouse_move(mlx_ptr, win_ptr, x, y);
    }
}

pub fn mouse_hide(mlx_ptr: *mut c_void, win_ptr: *mut c_void) {
    extern "C" {
        fn mlx_mouse_hide(mlx_ptr: *mut c_void, win_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_mouse_hide(mlx_ptr, win_ptr);
    }
}

pub fn mouse_show(mlx_ptr: *mut c_void, win_ptr: *mut c_void) {
    extern "C" {
        fn mlx_mouse_show(mlx_ptr: *mut c_void, win_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_mouse_show(mlx_ptr, win_ptr);
    }
}

pub fn event_loop(mlx_ptr: *mut c_void) {
    extern "C" {
        fn mlx_loop(mlx_ptr: *mut c_void) -> i32;
    }

    unsafe {
        mlx_loop(mlx_ptr);
    }
}

pub fn mouse_hook<T>(win_ptr: *mut c_void, cb: T)
where
    T: FnMut(i32, i32, i32) + 'static,
{
    extern "C" {
        fn mlx_mouse_hook(
            win_ptr: *mut c_void,
            func_ptr: unsafe extern "C" fn(i32, i32, i32, *mut c_void),
            param: *mut c_void,
        ) -> i32;
    }

    unsafe extern "C" fn call_closure<F>(buttons: i32, x: i32, y: i32, data: *mut c_void)
    where
        F: FnMut(i32, i32, i32),
    {
        let callback_ptr = data as *mut F;
        let callback = &mut *callback_ptr;
        callback(buttons, x, y);
    }

    let callback = Box::into_raw(Box::new(cb));
    unsafe {
        mlx_mouse_hook(win_ptr, call_closure::<T>, callback as *mut c_void);
    }
}

pub fn key_hook<F>(win_ptr: *mut c_void, cb: F)
where
    F: FnMut(i32) + 'static,
{
    extern "C" {
        fn mlx_key_hook(
            win_ptr: *mut c_void,
            func_ptr: unsafe extern "C" fn(i32, *mut c_void),
            param: *mut c_void,
        ) -> i32;
    }

    unsafe extern "C" fn call_closure<F>(keycode: i32, data: *mut c_void)
    where
        F: FnMut(i32),
    {
        let callback_ptr = data as *mut F;
        let callback = &mut *callback_ptr;
        callback(keycode);
    }

    let callback = Box::into_raw(Box::new(cb));
    unsafe {
        mlx_key_hook(win_ptr, call_closure::<F>, callback as *mut c_void);
    }
}

pub fn expose_hook<F>(win_ptr: *mut c_void, cb: F)
where
    F: FnMut() + 'static,
{
    extern "C" {
        fn mlx_expose_hook(
            win_ptr: *mut c_void,
            func_ptr: unsafe extern "C" fn(*mut c_void),
            param: *mut c_void,
        ) -> i32;
    }

    unsafe extern "C" fn call_closure<F>(data: *mut c_void)
    where
        F: FnMut(),
    {
        let callback_ptr = data as *mut F;
        let callback = &mut *callback_ptr;
        callback();
    }

    let callback = Box::into_raw(Box::new(cb));
    unsafe {
        mlx_expose_hook(win_ptr, call_closure::<F>, callback as *mut c_void);
    }
}

pub fn loop_hook<F>(win_ptr: *mut c_void, cb: F)
where
    F: FnMut() + 'static,
{
    extern "C" {
        fn mlx_loop_hook(
            win_ptr: *mut c_void,
            func_ptr: unsafe extern "C" fn(*mut c_void),
            param: *mut c_void,
        ) -> i32;
    }

    unsafe extern "C" fn call_closure<F>(data: *mut c_void)
    where
        F: FnMut(),
    {
        let callback_ptr = data as *mut F;
        let callback = &mut *callback_ptr;
        callback();
    }

    let callback = Box::into_raw(Box::new(cb));
    unsafe {
        mlx_loop_hook(win_ptr, call_closure::<F>, callback as *mut c_void);
    }
}

pub fn hook<F>(win_ptr: *mut c_void, x_event: i32, x_mask: i32, cb: F)
where
    F: FnMut(),
{
    extern "C" {
        fn mlx_hook(
            win_ptr: *mut c_void,
            x_event: i32,
            x_mask: i32,
            func_ptr: unsafe extern "C" fn(*mut c_void),
            param: *mut c_void,
        ) -> i32;
    }

    unsafe extern "C" fn call_closure<F>(data: *mut c_void)
    where
        F: FnMut(),
    {
        let callback_ptr = data as *mut F;
        let callback = &mut *callback_ptr;
        callback();
    }

    let callback = Box::into_raw(Box::new(cb));
    unsafe {
        mlx_hook(
            win_ptr,
            x_event,
            x_mask,
            call_closure::<F>,
            callback as *mut c_void,
        );
    }
}
