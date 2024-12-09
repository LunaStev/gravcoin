use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/bitcoin.cpp");

    let status = Command::new("make")
        .arg("all")
        .status()
        .expect("failed to execute make");

    if !status.success() {
        panic!("C++ build failed");
    }
}
