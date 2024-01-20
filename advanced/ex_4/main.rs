
fn last_element<T: Clone>(list : &[T]) -> T {
    let mut value : T = list[0].clone();
    for element in list.iter() {
            value = element.clone()
    }

    value
}

fn main() {
    let string_slice = ["Hello".to_string(), "world".to_string()];
    let i32_slice = [1, 2, 3];
    let char_slice = ['a', 'b', 'c'];
  
    println!("{}", last_element(&string_slice));
    println!("{}", last_element(&i32_slice));
    println!("{}", last_element(&char_slice));
  }