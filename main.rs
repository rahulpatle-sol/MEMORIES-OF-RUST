
use std::io;


fn main() {
    println!("Dear-user guss a  number! ");
    println!("Please enter your guss: ");
    let mut guss = String::new();
    io::stdio().read_line(&mut guss).expect("Error reading line");
    println!("you  gussed :{}", guss);
}
