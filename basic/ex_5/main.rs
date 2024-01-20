fn evaluate_number(number: i32) {
    if number == 0 {
        println!("Zero");
    }
    if number < 0 {
        println!("Negative");
    }
    if number > 0 {
        println!("Positive");
    }
}

fn main() {
  evaluate_number(0);
  evaluate_number(5);
  evaluate_number(-5);
}