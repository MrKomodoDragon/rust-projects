use std::io;

fn main() {
    //defining the string to use for input
    let mut input = String::new();
    println!("What would number would you like to convert to Farenheit?");
    //read the input
    io::stdin().read_line(&mut input).expect("Not a valid string");
    //parse the input
    let _input_num: f64 = input.trim().parse().expect("Not a valid float");
    //actual conversion
    let number: f64 = _input_num * 9.0/5.0 + 32.0; 
    //printing the output
    println!("{} degrees celsius is equal to {} degrees farenheit", _input_num, number);

}