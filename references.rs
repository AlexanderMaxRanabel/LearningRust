fn main() {
  
   let mut y = 420;
    
   let yr = &mut y;
    
    *yr += 1;
    
   println!("y is {}", yr);
    
}
