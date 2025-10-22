#[cfg(not(debug_assertions))]
use std::process::Command;

fn main() {
    // // Run when in release mode
    // #[cfg(not(debug_assertions))]
    // {
    //     // tailwindcss -i style/input.css -o style/output.css
    //     Command::new("tailwindcss")
    //         .args(["-i", "style/input.css", "-o", "style/output.css", "-m"])
    //         .spawn()
    //         .expect("Failed to run tailwindcss");
    // }
}
