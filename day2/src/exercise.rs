// ✅ Exercise: Level 1
// Declare a variable named my_age and set it to your age.


// Print the value of my_age to the console.
// Create a mutable variable named my_height and assign it your height in centimeters. Update it to a new height.
// Declare a variable my_name and assign it your name as a string. Print it to the console.
// Create a variable is_student and set it to true if you are a student, or false otherwise. Print the value.
// Create a variable birth_year and calculate your birth year by subtracting your age from the current year (you can use a hardcoded current year, e.g., 2024). Print the value.


// ✅ Exercise: Level 2
// Create variables for each numeric type (integer and float) and print their values:
// An integer variable my_integer set to any integer value.
// A floating-point variable my_float set to any float value.
// Declare a boolean variable is_learning_rust and set it to true. Print the value.
// Create a character variable favorite_letter and assign it your favorite letter. Print it.
// Create an array of integers called my_scores that holds your last five test scores. Print the entire array.
// Create a string variable hobby and assign it one of your hobbies. Print it, and then concatenate it with another string to create a sentence (e.g., "I enjoy [hobby]!"). Print the complete sentence.



fn main() {
    // exercise level ! 🎚️
	let  mut my_age: u8 = 21;
    println!("my age is :{}",my_age);
    my_age +=1;
    println!("my age after one year is :{}", my_age);
    let mut height:f32 =167.6;
    println!("my height is :{} cm ", height);
    height +=2.67;
    println!("my height after one year is :{} cm", height);
    let study_status:bool= true;
    if study_status==true{
        println!("I am a student.");
    } else {
        println!("I am not a student.");
    }
    let birth_year:u16=2025- my_age as u16;
    println!("My birth year is: {}", birth_year);
    // exercise level 2 🎚️
    // numeric types 
    let the_number:u8=45;
    let the_number_u16:u16= 1000;
    let _the_number_u32:u32= 34689;
    let the_number_u64:u64= 3469;
    let the_number_f32:f32= 45.67;
    let the_number_f64:f64= 45.67;
    println!("The integer number is only u: {},{},{},{}", the_number,the_number_u16,the_number_f32,the_number_u64 );
    println!("The float number is only f: {},{}", the_number_f32,the_number_f64);
let _my_integer:u16=756;
let _my_float:f64= 45.67;
let _is_learning_rust:bool = true;

let favorite_letter:char='m';
println!("My favorite letter is: {}", favorite_letter);

let my_game_scores: [u16; 5] = [85, 90, 78, 88, 92];
println!("My last five game scores are: {:?}", my_game_scores);
let aaata:[u32;3]=[1,2,3];
println!("the array is :{:?}",aaata);
let hobbies:&str="reading books";
println!("I enjoy {}!", hobbies);

}