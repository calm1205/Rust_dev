pub fn _run() {
  let s1 = String::from("hello");
  let _s2 = s1;
  // println!("{} {}", s1, s2); // この時点でs1の所有権がs2にmove。s1のデータにはアクセス出来ない。

  let i1 = 1;
  let i2 = i1; // 整数型はcopy traitが実装されている。別のstackにデータが保存される。shallow copy
  println!("{} {}", i1, i2); // この時点でs1の所有権がs2にmove。s1のデータにはアクセス出来ない。
  println!("Stack address of i1 is: {:p}", &i1);
  println!("Stack address of i2 is: {:p}", &i2);

  let sl1 = "literal";
  let sl2 = sl1;
  println!("{} {}", sl1, sl2); // 文字列スライスもshallow copy。 stackのpointerを参照しているだけなので参照渡し
  println!("Stack address of sl1 is: {:p}", &sl1);
  println!("Stack address of sl2 is: {:p}", &sl2);

  // deap copy
  let s3 = String::from("hello");
  let s4 = s3.clone(); // heap内の別のアドレスにデータがcopyされる。
  println!("Stack address of s3 is: {:p}", &s3);
  println!("Stack address of s4 is: {:p}", &s4);
  println!("Heap memory address of s3 is: {:?}", &s3.as_ptr()); // 複雑な形式のデータは{:?}で表示。格納されているポインターを参照する場合はas_prt()
  println!("Heap memory address of s4 is: {:?}", &s4.as_ptr());
  println!("{} {}", s3, s4); // 所有権のmoveが起きていないので両方活用できる。

  // 関数での所有権のmove
  // Rustの関数での所有権の移動タイミングは、引数に値を渡したとき。関数から値が返されたとき。
  let s5 = String::from("hello");
  println!("Stack address of s5 is: {:p}", &s5);
  println!("Heap address of hello: {:?}", &s5.as_ptr());
  println!("Len is: {}", s5.len());
  println!("Cap is: {}", s5.capacity());
  // take_ownership(s5);
  // println!("{}", s5); // s5は関数の引数として渡して所有権がmoveしたので活用出来ない。

  let s6 = String::from("hello");
  println!("Stack address of s6 is: {:p}", &s6);
  println!("Heap address of hello: {:?}", &s6.as_ptr());
  println!("Len is: {}", s6.len());
  // let s7 = take_giveback_ownership(s6); // s6 -> s -> s7と所有権はmove. 24byteのString型が２回移動
  // println!("Stack address of s7 is: {:p}", &s7);
  // println!("Heap address of hello: {:?}", &s7.as_ptr());
  // println!("Len is: {}", s7.len());

  // 関数で所有権のmoveが発生しないような使い方
  // let s8 = String::from("hello");
  // let len = calculate_length(&s8);
  // println!("The length of '{}' is {}", s8, len);

  // let mut s9 = String::from("hello");
  // change(&mut s9);
  // println!("{}", s9);

  // mutatbleなreferenceは１つまで。imutatbleなreferenceは複数生成できる。
  let s10 = String::from("hello");
  let r1 = &s10;
  let r2 = &s10;
  println!("{} {} {}", s10, r1, r2);

  // let mut s10 = String::from("hello");
  // let r1 = &s10;
  // let r2 = &mut s10; // imutableなreferenceが存在しているとmutableなreferenceは定義できない。
  // println!("{} {} {}", s10, r1, r2);

  // ======== mutableなreferenceの有効期限 =========
  // let mut s11 = String::from("hello");
  // let r11 = &mut s11;
  // println!("{}", s11); // error 例え最初に定義された所有権持ちでもmutableな範囲（mutable定義 ~ 最後に使うまで）では参照出来ない
  // println!("{}", r11);
}

// fn take_ownership(s: String) {
//   println!("Stack address of s5 is: {:p}", &s);
//   println!("Heap address of hello: {:?}", &s.as_ptr()); //所有権のmoveすなわちshallow copyなのでHeapのアドレスはs5が参照しているものと同じ
//   println!("Len is: {}", s.len());
//   println!("Cap is: {}", s.capacity());
//   println!("{}", s);
// }
// fn take_giveback_ownership(s: String) -> String {
//   s // Rustの関数は最後の行+セミコロン無しがreturn
// }
// // 引数で参照を受け取る
// fn calculate_length(s: &String) -> usize {
//   s.len()
// }
// fn change(s: &mut String) {
//   s.push_str("_world");
// }
