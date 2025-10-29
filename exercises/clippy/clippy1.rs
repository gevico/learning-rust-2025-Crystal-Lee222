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




fn main() {
    let pi = std::f32::consts::PI; // ✅ 使用标准库定义的精确 PI
    let radius = 5.00f32;             // ✅ 移除多余的 f32 后缀

    let area = pi * radius.powi(2); // ✅ 更简洁的写法

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}