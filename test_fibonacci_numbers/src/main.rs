use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number of fibonacci numbers to generate: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Please type a number!");

    println!("The fibonacci numbers are: {}", fibonacci(n));
}

fn fibonacci(n: u32) ->  u64{
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
