// //  rust if else if else if else
// ✅ Exercise: Level 1
// Write a program that checks if a number is even or odd using the if statement.
// Create a while loop that prints numbers from 1 to 10.
// Use the for loop to iterate over an array of your favorite colors and print each one.
// Create a simple calculator using the match statement that performs addition, subtraction, multiplication, or division based on user input.
// Write a program that continuously takes user input until the word "exit" is typed, using a loop.

use std::io;
fn main(){
    let num=10;
    if num%2==0{
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }

    let mut i:u32=0;
    while i <10{
        println!("{}", i);
        i += 1;
    }
    let colors = ["red", "green", "blue", "yellow"];
    for color in colors.iter() {
        println!("{}", color);
    }

    let mut choice = String::new();
    println!("Enter operation (+, -, *, /): ");
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Enter first number: ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    println!("Enter second number: ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");  
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");
    match choice {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Cannot divide by zero");
            }
        },
        _ => println!("Invalid operation"),
    }

    loop {
        let mut input = String::new();
        println!("Type 'exit' to quit or enter something else: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        } else {
            println!("You entered: {}", input);
        }
    }
    println!("Program exited.");

}