fn is_none(option: Option<i32>) -> bool {
    if option == None {
        return true;
    }
    false
}

fn get_str<'a>(list: &'a [String], item : &'a str) -> Option<&'a String> {
    for elem in list {
        if elem == item {
            return Some(elem)
        }
    }
    return None
}

fn main() {
    println!("{}", is_none(Some(1)));
    println!("{}", is_none(None));

    println!("{:?}", get_str(&["Hello".to_string(), "World".to_string()], "World"));
}