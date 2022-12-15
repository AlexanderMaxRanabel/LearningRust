fn main() {
  print_numbers_to(50);
}

fn print_numbers_to(num: u32) {
    for n in 1..num {
        println!("{}", n);
    } 
}
