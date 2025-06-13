fn main (){
let name_user=String::from("rahul patle");
println!("the char / length of the string is: {}", get_str_lenght(name_user));
}


fn get_str_lenght(str:String)->usize{
    return str.chars().count()
}