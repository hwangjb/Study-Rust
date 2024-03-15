fn main() {
    let mut s = String::from("Hello world");
    let word = first_word(&s);
    s.clear(); // s를 clear하면 word는 더이상 유효하지 않다.

    println!("The first word is: {}", word);


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
