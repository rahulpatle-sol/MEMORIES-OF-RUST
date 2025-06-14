fn main() {
    let my_string = String::from("asdghfy is a good Rust programmer");
    match  find_first_char(my_string) {
        Some(index) => println!("The first 'a' is at index: {}", index),
        None => println!("There is no char  in the string"),
    }
    println!("");
}



fn find_first_char(s:String)->Option <i32> {
    
        for(index,character) in s.chars().enumerate() {
            if character=='y'{
                return Some(index as i32);
            }
        }return None;
    }
    
