//  function  is starting point of the program


fn  main(){
  println!("this is a simple calculator in rust for ");
  println!("addition, subtraction, multiplication and division");


  println!("Enter operation (+, -, *, /):{} ",addition(20,40));
  println!("Enter operation (+, -, *, /):{} ",subtraction(20,40));
  println!("cube of 3 is: {} ",cube(3));
}
fn addition(a: i32, b: i32) -> i32 {
    a + b

}
fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}
fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}
fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
fn cube(a:i32,)->i32{
let cube=a*a*a;
return cube;
}