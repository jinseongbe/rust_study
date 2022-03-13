#[path = "types/integer.rs"] mod integer;
#[path = "types/char.rs"] mod char;
fn main() {
  integer::integer_print();  
  char::char_print();


  println!("done!")
}
