#[cfg(unix)]
#[path = "build/apple.rs"]
mod apple;

#[cfg(windows)]
#[path = "build/windows.rs"]
mod windows;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(unix)]
    match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("macos") => apple::build_macos(),
        Ok("ios") => apple::build_ios(),
        _ => {}
    }

    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        windows::build();
    }
}
