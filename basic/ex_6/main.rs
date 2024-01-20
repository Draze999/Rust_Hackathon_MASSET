fn print_slices_element(slice: &[String]) {
    for elem in slice.iter() {
        println!("{}", elem);
    }
}

fn main() {
  print_slices_element(&["Thomas".to_string(), "Nassim".to_string(), "Guillaume".to_string()]);
}