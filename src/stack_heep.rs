// enum List {
//   Node(i32, Box<List>),
//   Nil,
// }

pub fn _run() {
  //符号なし8bitsの7,000,000のlengthの配列
  // 配列は8MBまでしか容量を持てない。
  let _a1: [u8; 7_000_000] = [1; 7_000_000];
  // let a1: [u8; 9_000_000] = [1; 9_000_000]; // stack overflow

  // vector型 -> 可変の配列
  let mut v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  let mut v3 = vec![9, 10];
  println!("========== vector ==============");
  println!("Stack address of v1 is: {:p}", &v1);
  println!("Stack address of v2 is: {:p}", &v2);
  println!("Heep memory address of v1 is: {:p}", &v1);
  println!("Len of v1 is {}", v1.len()); // vectorのlengthは要素数になる。bitsではない。
  println!("Capacity of v1 is: {}", v1.capacity());

  v1.insert(1, 10); // 1番目の場所に10という数値を代入
  println!("{:?}", &mut v1);
  println!("Capacity of v1 is: {}", v1.capacity());

  v1.remove(0); // 0番目の値を削除
  println!("{:?}", &v1);

  // v1にv3を連結。
  v1.append(&mut v3);
  println!("{:?}", &v1);
  println!("{:?}", &v3); // -> [] 連結後の配列は空になる。

  // ============= box pointer =============
  // boxpinter -> Heapにデータを移行してそのpointerを保持する。
  println!("======= box pointer =============");

  let t1: (i64, String) = (10, String::from("hello"));
  println!("Stack address of tuple data t1 is: {:p}", &t1);
  println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr());
  println!("Len of t1.1: {:?}", t1.1.len());
  println!("Capacity of t1.1: {:?}", t1.1.capacity());

  let mut b1 = Box::new(t1);
  (*b1).1 += "world"; // 参照外し。t1.1のメモリのアドレスを直接見に行く
  println!("{} {}", b1.0, b1.1);
  println!("Stack address of box pointer b1 is: {:p}", &b1);
  println!("Heap address of tuple data b1 is: {:p}", b1);
}
