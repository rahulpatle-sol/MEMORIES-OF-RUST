// this code contain  conditional  exercise 
fn main(){
// execise 1 
//  tea is hot cool or drinkable 

// let temp:i8=75;
// if temp==60{
//   println!("tea is for drink")
// } else if temp<70{
//     println!("tea is modrate")
// }
// else if temp>70 && temp<90{
//     println!("tea is hot")
// }
// else {
//     println!("tea is cool")
// }

// execise 2
let wallet = 60;
let price = 50;
if wallet >= price {
    println!("bro you have enough money to buy the burger")

}
else if wallet<price && wallet>0{
    println!("bro you have not enough money to buy the burger")
}
else if wallet==0{
    println!("bro you have no money to buy the burger")
}
else {
    println!("bro you have no money to buy the burger")
}
// exercise 3--- raincoat chaeck
let rain:bool=true;
if rain==true{
    println!("you need to wear a raincoat")
}
else if rain==false{
    println!("you don't need to wear a raincoat")
}
else {
    println!("you don't need to wear a raincoat")


}

// exercise -4 even or odd number

let num:i32=10;
if num%2==0{
    println!("the number is even")
}
else if num%2!=0{
    println!("the number is odd")
}
else {
    println!("the number is odd")
}

// exercise 5  time of class

let time:i32=10;
if time==8{
    println!("the class is started")
}
else if time>8 && time<10{
    println!("the class is not started")
}
else if time==10{
    println!("the class is started and you are late");

}
else {
    println!("the class is not started")
}




}