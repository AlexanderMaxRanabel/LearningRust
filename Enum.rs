enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn main() {
  let player_direction:Direction = Direction::Down;
    
    match player_direction {
        Direction::Up => println!("You are going up"),
        Direction::Down => println!("You are going Down "),
        Direction::Left => println!("You are going Left"),
        Direction::Right => println!("You are going Right"),       
        
        
    }
}
