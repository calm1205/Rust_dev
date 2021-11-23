pub fn run() {
  // 変数の定義
  println!("======== variable define =========");
  let i = 100;
  println!("variable i is {}", i);

  // 符号付きの整数値
  println!("======== int variable define =========");
  let int1: i8 = 127; // i8: +/-有りの8bitの整数値（-128 ~ + 127）
  println!("variable int1 is {}", int1);
  // let int2: i8 = 128; // error: 8bitで表現できる範囲を超えているため

  // 符号なしの整数値
  let uint1: u8 = 255; //u8: +/-無し。（0~255）
  println!("variable uint1 is {}", uint1);
  // let uint2: u8 = 256; // error

  // 変数定義
  let mut int2: i64 = 1_000_000;
  println!("variable int2 is {}", int2);
  int2 = 2_000_000;
  println!("variable int2 is {}", int2);

  // stackとheap
  println!("======== stack & heap =========");
  let mut ss1 = String::from("hello");
  let mut ss2 = String::from("helloworld");
  println!("Stack address of ss1 is: {:p}", &ss1);
  println!("Stack address of ss2 is: {:p}", &ss2);
  println!("Heep memory address of ss1 is: {:?}", &ss1.as_ptr());
  println!("Heep memory address of ss2 is: {:?}", &ss2.as_ptr());
  ss1.push_str("_new1");
  ss2.push_str("_new2");
  println!("{} {}", ss1, ss2);
}
