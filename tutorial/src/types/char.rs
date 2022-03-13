use std::mem::size_of;

pub fn char_print() {
  // rust의 char는 4 bytes (거의 모든 문자 표현 가능)
  println!("Size of a char {}bytes", size_of::<char>());

  // rust char의 len은 글자수가 아니라 bytes수를 의미한다
  println!("\"a\".len() = {}", "a".len());
  println!("\"빅\".len() = {}", "빅".len());

  let letter = 'A';
  let space = ' ';
  let emoticon = '😊';
  println!("{}, {}, {}", letter, space, emoticon);

  // ASCII
  let ascii_num = 'a' as u8;
  println!("ascii_num 'a' is {}", ascii_num);

  println!();
}