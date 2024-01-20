fn element_at(slice: &[i32], index: usize) -> i32 {
    let mut value : i32 = 0;
    let mut ind : usize = 0;
    for element in slice.iter() {
        if index == ind {
            value = *element
        }
        ind += 1;
    }

    value
}

fn slices_len(slice: &[i32]) -> usize {
    let mut value : usize = 0;
    for element in slice.iter() {
        value += 1
    }

    value
}

fn main() {
  let numbers = [-9, 1, -2, 7, 1, 4, 6, -2, 7, -9];

  println!("{:?}", slices_len(&numbers));
  println!("{:?}", element_at(&numbers, 1));
}