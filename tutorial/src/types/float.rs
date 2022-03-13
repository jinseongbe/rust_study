pub fn float_print() {
  // f32, f64가 있지만 보통 f64로 사용함
  println!("{} =< f32 =< {}", f32::MIN, f32::MAX);
  println!("{} =< f64 =< {}", f64::MIN, f64::MAX);

  let num1 = 9.6;
  let num2 = 9;
  println!("9.6 + 9 = {}", num1 + num2 as f64);

  println!();
}