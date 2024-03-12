fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];

    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("The value of a is: {:?}", a);
}

/*
integer types
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
arch	isize	usize

Number literals Example
Decimal         98_222
Hex	            0xff
Octal	        0o77
Binary	        0b1111_0000
Byte(u8 only)	b'A'
*/
