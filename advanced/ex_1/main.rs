fn print_counter (n : &u32) {
    println!("{}", n)
}

fn increment_counter (n : &mut u32) {
    *n += 1
}

fn main() {
    let mut counter = 0u32;
  
    while counter < 5 {
      print_counter(&counter);
      increment_counter(&mut counter);
    }
  }