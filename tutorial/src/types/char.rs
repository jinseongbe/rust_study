pub fn char_print() {
  // rust의 char는 4 bytes (거의 모든 문자 표현 가능)

  let letter = 'A';
  let space = ' ';
  let emoticon = '😊';
  println!("{}, {}, {}", letter, space, emoticon);

  // ASCII
  let ascii_num = 'a' as u8;
  println!("ascii_num 'a' is {}", ascii_num);

  println!();
}