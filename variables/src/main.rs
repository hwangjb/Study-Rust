fn main() {
    // let x=5;
    // println!("The value of x is: {}", x);
    // x=6; // error: cannot assign twice to immutable variable `x`
    // println!("The value of x is: {}", x);

    // let mut x=5;

    // println!("The value of x is: {}", x);

    // x=6;

    // println!("The value of x is: {}", x);

    // Shadowing

    let x = 5;
    let x = x + 1; // x=6
    let x = x * 2; // x=12

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // spaces=3

    println!("The value of spaces is: {}", spaces);
}
