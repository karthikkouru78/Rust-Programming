5. Write a program to implement the following
a. Implicit type declaration
b. Explicit type declaration

fn main() {
    let x = 42;
    let y = 3.14;
    let z = "Rust";

    println!("Implicit types: x = {}, y = {}, z = {}", x, y, z);

    let a: i64 = 1000;
    let b: f32 = 2.718;
    let c: &str = "Explicit";

    println!("Explicit types: a = {}, b = {}, c = {}", a, b, c);
}
