pub fn string_print() {
  let txt = "sample string";
  let txt2 = "샘플 문자열";

  println!("\"{}\" is {}bytes(string.len()) and also {} characters(string.chars().count())", txt, txt.len(), txt.chars().count());
  println!("\"{}\" is {}bytes(string.len()) and also {} characters(string.chars().count())", txt2, txt2.len(), txt2.chars().count());

  println!()
}