struct Color {
    red: u8, //u8 0-255
    green: u8,
    blue: u8,
}

fn main() {
    
    let mut bg = Color {red: 243, green: 122, blue: 93};
    
    bg.green = 225;
    
    println!("{}, {}, {}", bg.red, bg.green, bg.blue)
    
}
