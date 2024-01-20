#[derive (Debug)]
enum Resultat<T, E> {
    Ok(T),
    Err(E),
 }

fn get_element_at<T: Clone>(list: &[T], x:usize) -> Resultat<T, String> {
    if x >= list.len()
        {return Resultat::Err("Index out of bounds".to_string())}
    
    Resultat::Ok(list[x].clone())
}

 fn main() {
    println!("{:?}", get_element_at(&["Hello".to_string(), "World".to_string()], 3));
    println!("{:?}", get_element_at(&["Hello".to_string(), "World".to_string()], 1));
}
