#[path = "types/integer.rs"] mod integer;
#[path = "types/char.rs"] mod char;
#[path = "types/string.rs"] mod string;
#[path = "types/float.rs"] mod float;

fn main() {
  integer::integer_print();  
  char::char_print();
  string::string_print();
  float::float_print();

  println!("done!")
}
