use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the Fahrenheit(F) or Celsius(C): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Enter the temperature: ");
    let mut temperature_string = String::new();
    io::stdin()
        .read_line(&mut temperature_string)
        .expect("Failed to read line");

    let temperature: f64 = temperature_string.trim().parse().expect("Please type a number!");

    println!("The temperature is: {}", temperature);

    if input.trim() == "F"{
        let c_templerature = (temperature - 32.0) * (5.0 / 9.0);
        println!("The temperature in Celsius is: {}", c_templerature);
    } else if input.trim() == "C" {
        let f_templerature = (temperature * 9.0 / 5.0) + 32.0;
        println!("The temperature in Fahrenheit is: {}", f_templerature);
    }
}
