use std::f64::consts::PI;
//The rectangle struct
struct Rectangle {
    width: f64,
    length: f64,
}
//The Circle Struct
struct Circle {
    radius: f64,
}
//The area trait
pub trait Area {
    fn area(&self) -> f64;
}

//Implementing the Area trait for rectangle
impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.length;
    }
}

//Implementing the Area trait for circle
impl Area for Circle {
    fn area(&self) -> f64 {
        return PI * self.radius.powf(2.0);
    }
}
//Main Function
fn main() {
    let rect = Rectangle {
        width: 9.0,
        length: 10.0,
    };
    println!("The area of the rectangle is: {}", rect.area());
    let circle = Circle { radius: 2.0 };
    println!("The area of the circle is: {}", circle.area());
}
