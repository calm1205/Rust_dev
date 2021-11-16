pub mod sub_a;
mod sub_b;

// constの値はスコープに関係なく定義可能。
const _MAX_POINTS: u32 = 100_000;

pub fn _run() {
  // println!("Here is run.");
  // sub_a::func_a();
  // sub_b::func_b();

  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // 使用する予定のない変数は_を先頭につけることでコンパイルエラーを回避できる。
  let _i1 = 3; // letは関数の外のスコープでは定義出来ない。
  let _f1 = 0.1;

  println!("{}", usize::BITS); // ::BITSオプションを使用することで出力をバイトにできる。

  // println!("Memory address of const  is: {:p}", &MAX_POINTS); // :pを使うことでポインターの形式で出力が可能

  let i2: i64 = 1;
  let i3: i64 = 2;
  let i4: i64 = 3;
  println!("Stack address of i2 is: {:p}", &i2);
  println!("Stack address of i3 is: {:p}", &i3);
  println!("Stack address of i4 is: {:p}", &i4);

  println!("=========");
  let y = 5;
  println!("Stack address of y  is: {:p}", &y);
  let y = y + 1;
  println!("Stack address of y is: {:p}", &y);
  let y = y * 2;
  println!("The value of y is: {}", y);

  // 同じ変数名でもカーリーブラケットでスコープを区切れば別物として扱うことができる。
  {
    let y = 0;
    println!("The value of y is: {}", y);
  }

  println!("The value of y is: {}", y);

  let t1 = (500, 6.4, "dummy");
  let (_x, _y, _z) = t1;
  println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
  *x1_ptr = 5; // 参照外し。*pointerでそのポインターが指し示す具体値にアクセスできる。
  *y1_ptr = -5;
  println!("{:?}", t2);

  let a1 = [1, 2, 3, 4, 5];
  let a2 = [0; 10]; // = [0,0,0,0,0,0,0,0,0,0]
  println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

  let u1: u8 = 255; // 8bit つまり256パターン。つまり0~255。
  let mut _i5: i8 = 127; // 8bitで+-も表現。つまり-128~127。
  println!("{}", u1);

  println!("======= 文字列スライスとstring型 ===========");
  // 半各ローマ字1文字1byte、全角は1文字3byte
  let s1 = "helloこんにちは挨拶"; //26bytes
  let s2 = "hello";
  println!("Stack address of s1 is: {:p}", &s1);
  println!("Stack address of s2 is: {:p}", &s2);

  println!("Stack memory address of s1: {:?}", &s1.as_ptr());
  println!("Stack memory address of s2: {:?}", &s2.as_ptr());

  println!("Len of s1 is {}", &s1.len());
  println!("Len of s2 is {}", &s2.len());

  let mut ss1 = String::from("hello");
  let mut ss2 = String::from("helloworld");
  println!("Stack address of ss1 is: {:p}", &ss1);
  println!("Stack address of ss2 is: {:p}", &ss2);
  println!("Heep memory address of ss1 is: {:?}", &ss1.as_ptr());
  println!("Heep memory address of ss2 is: {:?}", &ss2.as_ptr());
  println!("Len of ss1 is {}", &ss1.len());
  println!("Len of ss2 is {}", &ss2.len());
  println!("Capacity of ss1 is: {:?}", &ss1.capacity());
  println!("Capacity of ss2 is: {:?}", &ss2.capacity());
  ss1.push_str("_new1");
  ss2.push_str("_new2");
  println!("{} {}", ss1, ss2);
}
