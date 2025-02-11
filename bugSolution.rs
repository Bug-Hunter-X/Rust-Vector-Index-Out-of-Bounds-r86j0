fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(10);
    match number {
        Some(n) => println!("The value is {}", n),
        None => println!("Index out of bounds"),
    }

    // Safer way to access elements within bounds:
    if let Some(n) = numbers.get(1) {
        println!("The value at index 1 is: {}", n);
    }
} 