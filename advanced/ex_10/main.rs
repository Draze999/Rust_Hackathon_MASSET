use std::collections::HashMap;

fn create_hash_map(input: &[&str]) -> HashMap<String, i32> {
    let mut histo = HashMap::<String, i32>::new();
    for elem in input {
        if histo.contains_key(&elem.to_string()) {
            histo.insert(elem.to_string(), histo.get(&elem.to_string()).unwrap() + 1);
        }
        else {
            histo.insert(elem.to_string(), 1);
        }
    }
    histo
}

fn main() {
    println!("{:?}", create_hash_map(&["Thomas", "Thomas", "Guillaume"]));

}