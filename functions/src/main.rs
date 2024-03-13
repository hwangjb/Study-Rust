fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5);
    anorther_function2(5, 6);

    let y={
        let x=3;
        x+1
    };
    println!("The value of y is: {}", y);

    println!("The value of return_fucntion is: {}", return_fucntion(y+2));
}

fn another_function(){
    println!("Another function.");
}

fn another_function1(x: i32){
    println!("The value of x is: {}", x);
}

fn anorther_function2(x:i32, y:i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_fucntion(x:i32)->i32{
    x+1
}