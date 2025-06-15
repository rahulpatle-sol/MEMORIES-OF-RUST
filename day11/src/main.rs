use chrono::{UTC, Local};

fn main() {
  let local=Local::now();
    println!("Current   local time: {}",local);
    let utc_time=UTCtc::now();
    let utc_time=utc_time.format("%Y-%m-%d %H:%M:%S");
    println!("Current UTC time: {}", utc_time);
}
//  packege managment in rust 
// Cargo is the package manager and build system for Rust.
// It helps manage dependencies, build packages, and run tests.
// To create a new Rust project, use the command:
// cargo  add <package_name>
// cargo  add chrono
