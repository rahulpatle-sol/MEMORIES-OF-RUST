// //  rust if else if else if else
// ✅ Exercise: Level 1
// Write a program that checks if a number is even or odd using the if statement.
// Create a while loop that prints numbers from 1 to 10.
// Use the for loop to iterate over an array of your favorite colors and print each one.
// Create a simple calculator using the match statement that performs addition, subtraction, multiplication, or division based on user input.
// Write a program that continuously takes user input until the word "exit" is typed, using a loop.

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
    }let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }
    
}