use std::process::Command;

// Run `cargo run --example all`
fn main() {
    Command::new("cargo")
        .args(["run", "--example", "all"])
        .status()
        .expect("failed to execute process");
    println!("Success");
}
