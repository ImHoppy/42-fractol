use std::process::Command;

fn main() {
    let status = Command::new("make")
        .current_dir("../minilibx")
        .arg("all")
        .env("CFLAGS", "-fPIC")
        .status()
        .expect("Failed to execute make command");

    if !status.success() {
        panic!("Failed to execute make command");
    }

    println!("cargo:rustc-link-search=./minilibx/");
    println!("cargo:rustc-link-lib=mlx");
    println!("cargo:rustc-link-lib=Xext");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=m");
}
