4. Write a program to implement the Scope and Shadowing


fn main() {
    let a = 10;
    println!("Outer a: {}", a);

    {
        let a = "hello";
        println!(" Inner a (string): {}", a);

        let a = true;
        println!(" Inner a (bool): {}", a);
    }

    println!("Outer a after block: {}", a);

    let a = a + 5;
    println!("Outer a after shadowing: {}", a);
}
