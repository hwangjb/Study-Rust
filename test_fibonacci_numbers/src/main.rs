use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number of fibonacci numbers to generate: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Please type a number!");

    let mut a: u64 = 1;
    let mut b: u64 = 1;
    let mut c: u64 = 0;
    for i in 1..n + 1 {
        if i == 1 || i == 2 {
            //println!("The fibonacci number {} is: 1", i);
        } else {
            c = a + b;
            //println!("The fibonacci number {} is: {}", i, c);
            a = b;
            b = c;
        }
    }
    //println!("The fibonacci numbers are: {}", fibonacci(n));
    println!("The fibonacci numbers are: {}", c);
}

// fn fibonacci(n: u32) -> u64 {
//     if n == 1 || n == 2 {
//         1
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }
