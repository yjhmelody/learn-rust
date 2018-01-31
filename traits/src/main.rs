//! # The first line
//! The second line
/// Adds one to the number given.
///
/// # Examples
///
///

use std::cell::Cell;

fn main() {
    let circle = Circle {
        x: 1.0,
        y: 2.0,
        radius: 1f64,
    };

    let c = Circle::default();

    println!("{}", circle.area());
    
    foo(circle.area());
    foo2(circle.area());
    foo3(circle.x, circle.y);

    let a: i32 = 32;
    println!("i32 to f64: {:?}", a.area());
    println!("{:?}", Person2);

    for x in 1..3 {
        println!("{}", x);
    }
    for (k, v) in (5..10).enumerate() {
        println!("k = {}, v = {}", k, v);
    }
    let lines = "Content of line one
Content of line two
Content of line three
Content of line four".lines();
    for(k, line) in lines.enumerate() {
        println!("k:{} line:{}", k, line);
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
        }
    }

    // 和Golang一样，Rust的数组中的 N （大小）也是类型的一部分，即 [u8; 3] != [u8; 4]
    let arr2: [i32; 3] = [2,3,4];

    println!("{}", arr2[2]);

    // str 类型基本上不怎么使用，通常使用 &str 类型，它其实是 [u8] 类型的切片形式 &[u8] 。
    // 这是一种固定大小的字符串类型。 常见的的字符串字面值就是 &'static str 类型。这是一种带
    // 有 'static 生命周期的 &str 类型。
    // let hello = "hello, world!";
    let hello: &'static str = "hello world";
    println!("{}", hello);

    let mut x = String::new();
    let mut hello = String::from("Hello,");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", hello);



    let point = Point{x: 5, y: Cell::new(6)};
    point.y.set(7);
    println!("{:?}", point);

    let x: Message = Message::Move{x:3, y:4};
    println!("{:?}", x);

    //  Rust中的 if 是一个表达式(expression)，可以赋给一个变量
    let x = 5;
    let x = if x == 5 { 10 } else {15};
    println!("{}", x);

    // Rust是基于表达式的编程语言，有且仅有两种语句 (statement)：
    // 1. 声明语句 (declaration statement)，比如进行变量绑定的 let 语句。
    // 2. 表达式语句 (expression statement)，它通过在末尾加上分号 ; 来将表达式变成语句， 丢弃
    // 该表达式的值，一律返回unit () 。

    let pair = (0, -2);
    match pair {
        (0, y) => println!("{}", y),
        (x, 0) => println!("{}", x),
        _ => println!("not matched"),
    }


    let num = 5;
    let plus_num = |x: i32| x + num;
    // 其中闭包 plus_num 借用了它作用域中的 let 绑定 num 。如果要让闭包获得所有权， 可以使用 move 关键字

    println!("{}", plus_num(10));
    
}

// 在Rust中，一个 char 类型表示一个Unicode字符,这也就意味着，在某些语言里代表一个字符
// (8bit)的char，在Rust里实际上是四个字节(32bit)。 同时，我们可以将各种奇怪的非中文字符随心
// 所欲的赋值给一个char类型。需要注意的是，Rust中我们要用 ' 来表示一个char，如果用 " 的话
// 你得到的实际上是一个 &'static str 。
#[derive(Debug)]
struct Person2;

// 与结构体一样，枚举中的元素默认不能使用关系运算符进行比较 (如 == , != , >= )， 也不支持
// 像 + 和 * 这样的双目运算符，需要自己实现，或者使用 match 进行匹配。

// 枚举默认也是私有的，如果使用 pub 使其变为公有，则它的元素也都是默认公有的。 这一点是与
// 结构体不同的：即使结构体是公有的，它的域仍然是默认私有的。 此外，枚举和结构体也可以是递归的 (recursive)。

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move{x: i32, y: i32},
    Write(String),
}

// Rust有一个特殊特性适用于发散函数 (diverging function)，它不返回
// fn diverges () -> ! {
    // 其中 panic! 是一个宏，使当前执行线程崩溃并打印给定信息。返回类型 ! 可用作任何类型
    // panic!("never return");
    // return "diverges"
// }

#[derive(Debug)]
struct Point {
    x: i32,
    y: Cell<i32>,
}

trait HasArea {
    fn area (&self)->f64;
}

#[derive(Default)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}


// 泛型的trait约束
use std::fmt::Debug;
fn foo<T:Debug>(s: T) {
    println!("{:?}", s);
}

// 多trait约束
fn foo2<T: Debug + Clone>(s: T) {
    println!("{:?}", s);
}

// 约束的trait增加后，代码看起来就变得诡异了，这时候需要使用 where 从句
fn foo3<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// 内置类型如： i32 , i64 等也可以添加trait实现，为其定制一些功能
impl HasArea for i32 {
    fn area(&self) -> f64 {
    *self as f64
    }
}
// 这样的做法是有限制的。Rust 有一个“孤儿规则”：当你为某类型实现某 trait 的时候，必须要求类
// 型或者 trait 至少有一个是在当前 crate 中定义的。你不能为第三方的类型实现第三方的 trait 。

// 在调用 trait 中定义的方法的时候，一定要记得让这个 trait 可被访问


// trait的默认方法
trait Foo {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}
// is_invalid 是默认方法， Foo 的实现者并不要求实现它，如果选择实现它，会覆盖掉它的默认行为。


// trait的继承
trait FooBar : Foo {
    fn foobar(&self);
}
// 这样 FooBar 的实现者也要同时实现 Foo

// Rust提供了一个属性 derive 来自动实现一些trait，这样可以避免重复繁琐地实现他们，能
// 被 derive 使用的trait包括： Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd


fn make_pair<T1, T2>(t1: T1, t2: T2) -> (T1, T2) {
    (t1, t2)
}

