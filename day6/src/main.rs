

// struct is here to demonstrate how to use structs in Rust
fn main() {
   let ract1 = Ract {
        length: 10,
        width: 5,
    };
    let area = ract1.area_rect();
    let perimeter = ract1.paremeter_rect();
    println!("Area of rectangle is: {}", area);
    println!("Perimeter of rectangle is: {}", perimeter);
    println!("{}", Ract::debug());

}


///  struct ract in rust like class in javascript
// It has methods to calculate area and perimeter of rectangle
// and a debug method that returns a constant value.
struct Ract{
    length:u32,
    width:u32
}
impl Ract{
    fn area_rect(&self)->u32{
        self.length * self.width
    }
    fn paremeter_rect(&self)->u32{
        2 * (self.length + self.width)
    }
    fn debug()->u32{
        return  1;
    }
}