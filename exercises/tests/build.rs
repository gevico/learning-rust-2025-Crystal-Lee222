//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // Part 1: Set TEST_FOO for tests7
    // Use `cargo:rustc-env` so it's available at runtime via std::env::var
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Part 2: Enable "pass" feature for tests8
    println!("cargo:rustc-cfg=feature=\"pass\"");
}