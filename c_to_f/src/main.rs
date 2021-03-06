use std::io;
fn main() {
    let mut input = String::new();
    println!("What would number would you like to convert to Farenheit?");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let _input_num: f64 = input.trim().parse().expect("Not a valid float");
    let number: f64 = _input_num * 9.0/5.0 + 32.0; 
    println!("{} degrees celsius is equal to {} degrees farenheit", _input_num, number);

}