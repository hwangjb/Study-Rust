fn main() {
    let s = String::from("Hello world");
    let word_1 = first_word(&s);

    let hello = &s[0..5];
    let world = &s[6..11];

    let word_2 = second_word(&s);

    //s.clear(); // s를 clear하면 word는 더이상 유효하지 않다.

    println!("The first word is: {}", word_1);
    println!("The second word is: {}", word_2);

    println!("The first word is: {}", hello);
    println!("The second word is: {}", world);


    println!("-------------------------------------------------");

    let s2=String::from("Hello world");
    let s3 = "Hello world";
    let word_3=third_word(&s2);
    let word_4 = third_word(s3);

    println!("The third word is: {}", word_3);

    println!("The third word is: {}", word_4);
    
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // ascii 코드로 변환(바이트 배열로 변환)

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn third_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
