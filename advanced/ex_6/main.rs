mod book_mod {

  pub struct Book {
    name: String
  }

  impl Book {
    pub fn new (na:&str) -> Book {
        let res = Book {
            name : na.to_string()
        };
    
        res
      }
  }

  pub fn read_book (booky: &Book) {
    println!("{}", booky.name)
  }

}

fn main() {
    let my_awesome_book = book_mod::Book::new("My Incredible Book");

    book_mod::read_book(&my_awesome_book);
}
