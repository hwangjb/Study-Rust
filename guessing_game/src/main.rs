extern crate rand; // 외부 라이브러리 사용을 위한 선언

use std::io; // 표준 라이브러리의 io 라이브러리 사용을 위한 선언
use rand::Rng; // 외부 라이브러리의 Rng trait 사용을 위한 선언
use std::cmp::Ordering; // 표준 라이브러리의 cmp 라이브러리 사용을 위한 선언

fn main(){
    println!("Guess the number!");

    let secret_number=rand::thread_rng().gen_range(1,101); // 1~100 사이의 난수 생성
    //println!("The secret number is: {}", secret_number); // 생성된 난수 출력

    loop{
        let mut guess = String::new(); // 가변 변수 선언

        println!("Please input your guess.");

        io::stdin().read_line(&mut guess).expect("Failed to read line"); // 문자열 입력

        let guess: u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        }; // 입력받은 문자열이 숫자일 경우 u32로 변환, 아닐 경우 다시 입력

        println!("You guessed: {}", guess); // 입력받은 변수 출력


        match guess.cmp(&secret_number){ // 입력받은 변수와 생성된 난수 비교
            Ordering::Less=>println!("Too small!"), // 입력받은 변수가 작을 경우 출력
            Ordering::Greater=>println!("Too big!"), // 입력받은 변수가 클 경우 출력
            Ordering::Equal=>{
                println!("You win!"); // 입력받은 변수가 같을 경우 출력
                break; // 루프 종료
            }
        } 
    }

    // let x=5; //mut을 사용할 경우 가변 변수 이지만 let으로 단독 사용시 불변 변수
    // let y=10;
    // x=6; // error: cannot assign twice to immutable variable `x`

    // println!("The value of x is: {} and the value of y is: {}", x, y);

}