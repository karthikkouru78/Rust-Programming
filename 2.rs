Write a program to display Output following pattern using Placeholders
1
22
333
4444

fn main() {
    for i in 1..=4 {
        let line = format!("{}", i.to_string().repeat(i));
        println!("{}", line);
    }
}
