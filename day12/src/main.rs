fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    numbers.push(6);

    println!("Numbers: {:?}", numbers);

    if let Some(last) = numbers.pop() {
        println!("Popped: {}", last);
    }

    println!("Numbers after pop: {:?}", numbers);
}
