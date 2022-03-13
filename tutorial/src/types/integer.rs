pub fn integer_print() {
  // 변수명 앞에 underbar(_)를 붙이면 사용하지 않더라도 warning이 표시되지 않음  

  // rust는 다른 타입간의 연산을 하용하지 않음 : ex) i8과 u16은 연산 안됨 -> 에러
  // 그럴땐 casting을 하면 됨
  let num1: u16 = 8;
  let num2: u8 = 10; // 다음표기로도 사용 가능 let num2 = 10u8;, let num = 10_u8, 언더바는 무시되니 자유롭게 사용 가능
  let num3 = num1 + num2 as u16; // num2를 u16로 casting 하여 연산한다.
  println!("{}, {}, {}", num1, num2, num3);

  // 부호 포함, 음의 정수, 양의 정수
  let _i8: i8 = 6;       // 2^7 
  let _i16: i16 = 6;     // 2^15
  let _i32: i32 = 6;     // 2^31
  let _i64: i64 = 6;     // 2^63
  let _i128: i128 = 6;   // 2^127
  let _isize: isize = 6; // 컴퓨터의 운영체제에 따라 32bit = i32, 64bit = i64

  // 부호 제외, 양의 정수만
  let _u8: u8 = 6;       // 2^8
  let _u16: u16 = 6;     // 2^16
  let _u32: u32 = 6;     // 2^32
  let _u64: u64 = 6;     // 2^64
  let _u128: u128 = 6;   // 2^128
  let _usize: usize = 2; // 컴퓨터의 운영체제에 따라 32bit = u32, 64bit = u64

  println!("{} =< i8 =< {}", i8::MIN, i8::MAX);
  println!("{} =< i16 =< {}", i16::MIN, i16::MAX);
  println!("{} =< i32 =< {}", i32::MIN, i32::MAX);
  println!("{} =< i64 =< {}", i64::MIN, i64::MAX);
  println!("{} =< i128 =< {}", i128::MIN, i128::MAX);
  println!("{} =< isize =< {}", isize::MIN, isize::MAX);
  
  println!("{} =< u8 =< {}", u8::MIN, u8::MAX);
  println!("{} =< u16 =< {}", u16::MIN, u16::MAX);
  println!("{} =< u32 =< {}", u32::MIN, u32::MAX);
  println!("{} =< u64 =< {}", u64::MIN, u64::MAX);
  println!("{} =< u128 =< {}", u128::MIN, u128::MAX);
  println!("{} =< usize =< {}", usize::MIN, usize::MAX);
  println!()
}
