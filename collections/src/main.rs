fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    for i in &vector {
        println!("{}", i)
    }
}
