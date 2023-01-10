struct Color {
    red: u8, //u8 0-255
    green: u8,
    blue: u8,
}

fn main() {
    
    let mut x = Color {red: 243, green: 122, blue: 93};
    
    print_color(&x);
}

fn print_color(c: &Color){
    println!("Color - R:{}, G:{}, B:{}", c.red, c.green, c.blue);
}
