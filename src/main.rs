mod generics;
mod ownership;
mod stack;
mod stack_heep;
mod vars;

fn main() {
  // !はマクロを意味する。これがないとprintlnがメソッドとして認識されない
  // println!("Hello, world!");
  // vars::run();
  // vars::sub_a::func_a();
  // stack_heep::run();
  // ownership::run();
  generics::run();

  // stack::run();
}
