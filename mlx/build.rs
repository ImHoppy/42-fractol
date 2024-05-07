fn main() {
    println!("cargo:rustc-link-search=./minilibx/");
    println!("cargo:rustc-link-lib=mlx");
    println!("cargo:rustc-link-lib=Xext");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=m");
}
