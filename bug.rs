fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(10);
    match number {
        Some(n) => println!("The value is {}", n),
        None => println!("Index out of bounds"),
    }
}