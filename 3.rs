3. Write a program to do the following
a. Declare a variable x and store value 1000 in it.
b. Declare a variable y and store value “Programming” in it
c. Print the values of x and y
d. Change the value of x to 1100
e. Print the values of x and y

fn main() {
    let mut x = 1000;
    let y = "Programming";
    println!("Initial values -> x: {}, y: {}", x, y);
    x = 1100;
    println!("Updated values -> x: {}, y: {}", x, y);
}
