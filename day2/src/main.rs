fn main() {
    // let mut a = 100;
    // println!("Initial a: {}", a); // Now used
    // a = 400;
    // let b = 200;
    

    // let agree=true;
    // let dessgree=false;
    // println!("a: {}, b: {}", a, b);
    // println!("agree: {}, dessgree: {}", agree, dessgree);
    // println!("{}, {}", a, b);
    // const PI :f64=3.14;
    // println!("pi: {}", PI);
    // const MAXDAY:u8=7;
    // println!("maxday: {}", MAXDAY);// CONST SHOULD BE UPPERCASE


    let  mut rty:i32=2334;
    println!("value of rty : {rty} before ");
    rty=rty+234;
 println!("value of  after rty :{rty} ");

 // github repo revision 
 let name="github.com/yourusername/day2";
 let rating=4.8;
 let is_active=true;
 println!("Name: {}, Rating: {}, Active: {}", name, rating, is_active);
    // Using a tuple
    let icon="🚀";
    println!("Icon: {}", icon);
    println!("Name: {}, Rating: {}, Active: {}", name, rating, is_active);
    //  mutability and immiutability
let  mut x = 10; // x is immutable

x += 20; // x is mutable
let y = 30; // y is immutable
println!("x: {}, y: {}", x, y);

    // Using a constant
const MAX_USERS: u32 = 1000;
println!("Max users: {}", MAX_USERS);


//  Hands-On Challenge
// Create a Rust program that uses variables and demonstrates different data types. Your program should:

// Declare and print a variable holding your name.
let myname="Rahul patle";
println!("My name is: {}", myname);
// Create a mutable integer variable representing your current age and update it by adding one.
let mut myage:u8=21;
myage +=1;
println!("My age after one year: {}", myage);
// Use a floating-point variable to store your favorite number and print it.
let myfloat:f32=45.8900;
println!("my fav. floating number is:{}",myfloat);
// Include a boolean variable that indicates whether you are learning Rust (set it to true).\
let learning_rust:bool=true;
println!("Am I learning Rust? {}", learning_rust);
// Use a character variable to store the first letter of your name.
let f_char:char='r';
println!("First letter of my name is :{}", f_char);

}

