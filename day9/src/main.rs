fn main() {
match divide(100,45){
    Ok (result )=>{
        println!("Result is {}",result)
    }
    Err(e)=>{
        println!("Error: {}", e)
    }
}
}

///  Result Enum is used to handle errors in Rust.
/// 
fn divide(a:i32,b:i32)->Result<i32,String>{
    if b==0{
        Err (String::from("Cannot divide by zero"))
    }

    else{
        Ok(a/b)
    }
}