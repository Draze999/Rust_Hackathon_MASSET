fn add (n1 : i32, n2 : i32) -> i32 {
    n1 + n2
}

fn print_modulo (n1 : i32, n2 : i32) {
    let mut n1cop = n1;
    while n1cop >= n2 {
        n1cop -= n2
    }
    println!("{}", n1cop)
}

fn slice_sum (slice: &[i32]) -> i32 {
    let mut res = 0;
    for elem in slice.iter() {
        res += *elem
    }

    res
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
  }
  