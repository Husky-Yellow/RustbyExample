fn main() {
  // 变量可以被类型注释。
  let logical: bool = true;

  let a_float: f64 = 1.0;  // 常规的注释
  let an_integer   = 5i32; // 后缀注释

  // 或者使用默认值.
  let default_float   = 3.0; // `f64`
  let default_integer = 7;   // `i32`

  // 还可以从上下文推断类型
  let mut inferred_type = 12; // 类型i64是从另一行推断出来的
  inferred_type = 4294967296i64;

  // 可变变量的值是可以改变的。
  let mut mutable = 12; // Mutable `i32`
  mutable = 21;

  // 错误!变量的类型不能更改。
  mutable = true;

  // 可以用阴影覆盖变量。
  let mutable = true;
}