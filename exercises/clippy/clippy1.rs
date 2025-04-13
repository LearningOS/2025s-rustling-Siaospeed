// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.
use std::f32;

fn main() {
    let pi = f32::consts::PI; // 使用 Rust 内建的常量 PI
    let radius = 5.00f32;

    let area = pi * radius * radius; // 使用乘法代替 powi

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
