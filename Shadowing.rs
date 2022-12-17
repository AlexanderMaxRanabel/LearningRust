fn main() {
    let mut x = 50;
    
    {
       let x = 16;
    }
    
    let x = "Stringed";
    println!("x is {}", x)
    
}
