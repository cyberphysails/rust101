+ strings are UTF-8 encoded;
+ rust use macros instead of function overloading to solve situtations where function wants to have a variable number of arguments;
+ macros being hygienic https://veykril.github.io/tlborm/decl-macros/minutiae/hygiene.html

## Basics
### Variables

variables are immutables by default;
variables must be initialized before using;

```rust
let x: i32 = 10;
let mut y = 11;
```

type inference 类型推断中默认整型为 `i32` ,浮点型为 `f64`

```rust
const THING: i32 = 13;
```
constants 必须显式声明变量类型，rust 不会做类型推断；

`static` 变量拥有 `'static` 生命周期，即它在整个程序运行期间都有效，所以常常用于全局变量；`static` 变量也必须显示声明类型和赋予常量值或表达式，它代表内存中的一个固定位置；而 `const` 变量在编译时是 inline 的，并不一定指向内存中的一个固定位置；

### Values

+ signed integers: i8, i16, i32, i64, i128, isize; -10, 0, 0xFF, 0o11, 0b10, 1_000, 123_i64 (下划线仅是为了提高可读性, 可以放在数字中的任何位置)
+ unsigned integers: u8, u16, u32, u64, u128, usize;
+ floating point: f32, f64; 
+ booleans: bool; true, false; 内存宽度为 8 bits;
+ unicode scalar values: char; 'a' 使用单引号括起; 内存宽度为 32 bits;

浮点型在 print 时可以自动四舍五入，比如 `{:.2}` 会自动保存四舍五入到两位小数点后两位；

### Arithmetic

rust 中对整型溢出有特定的处理函数;

### Control Flow

在 rust 中 `if` 是一个语句，有返回值; `for`, `while`, `loop` 语句也相同；
```rust
let x = 10;
let res = if x > 5 { "large" } else { "small" };
```

`for` 语句对迭代器 iterator 进行迭代；

`loop` 语句是一个永久的循环，需要 `break` 语句终止; `break` 语句可以为 `loop` 返回值；

```rust
let a = 13;
let mut factor = 2;
let res = loop {
    if (a % factor) == 0 {
        break false;
    };
    factor += 1;
    if ( a - 1 ) == factor {
        break true;
    };
};

println!("{a} is a prime number? {res}");
```

### Array and Tuple

rust 中数组和元组都是固定长度的，元组中可以包含不同类型的元素，而数组元素必须是相同类型；

数组和元组取元素时的表达式也不同，`array[index]` `tuple.index`; 元组可以解构取值，数组不可以；数组可以用 `len()` 函数返回其长度，元组不行；
