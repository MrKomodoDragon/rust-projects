struct Rectangle {
    width: i32,
    length: i32,
}

pub trait Area {
    fn area(&self) -> i32;
}

impl Area for Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.length
    }
}

fn main() {
    let rect = Rectangle {
        width: 9,
        length: 10,
    };
    let thing = 90;
    println!("The area of the rectangle is: {}", rect.area());
    println!("{}", &thing)
}
