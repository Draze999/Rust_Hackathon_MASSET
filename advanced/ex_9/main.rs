
fn remove_first_and_last(vec:Vec<i32>) -> Vec<i32> {
    let mut newvec = vec.clone();
    newvec.pop();
    newvec.reverse();
    newvec.pop();
    newvec.reverse();

    newvec
}

fn concat_vec(mut vec1:Vec<i32>, mut vec2:Vec<i32>) -> Vec<i32> {
    vec1.append(&mut vec2);
    let mut newvec = Vec::<i32>::new();
    for elem in vec1 {
        newvec.push (2 * elem)
    }

    newvec
}


fn main() {
    println!("{:?}", remove_first_and_last(vec![1, 2, 3, 4, 5]));
    println!("{:?}", concat_vec(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]));
  }