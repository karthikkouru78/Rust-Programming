9. Write a program to create different types of constants print it in the output

fn main() {
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.1415926535;
    const GREETING: &str = "Hello, Rust!";

    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("PI: {}", PI);
    println!("GREETING: {}", GREETING);
}
