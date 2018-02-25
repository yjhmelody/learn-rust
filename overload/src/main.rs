use std::ops::*;


#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex{a: self.a + other.a, b: self.b + other.b}
    }
}

impl Add<i32> for Complex {
    type Output = f64;
    fn add(self, other: i32) -> f64 {
        self.a + self. b + (other as f64)
    }
}



trait HasArea<T> {
    fn area(&self) -> T;
}

#[derive(Debug)]
struct Square <T>{
    x: T,
    y: T,
    side: T,
}

impl <T> HasArea<T> for Square<T>
    where T: Mul<Output=T> + Copy
{
    fn area(&self) -> T {
        self.side * self.side
    }
}

fn add<T:Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

// 我们知道了每一个引用都有一个生命周期，而且需要为使用了引用的函数或结构体指定生命周期。
#[derive(Debug)]
// 一个存放引用的结构体，所以其定义需要生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn foo<'a>(a: i32, s: &'a str) -> &'a str {
    
    println!("{}", s);
    println!("{}", a);
    s
}

fn main() {
    let c1 = Complex{a: 1f64, b:2.0};
    let c2 = Complex{a: 2f64, b:-1.0};
    let c3 = c1 + c2;
    println!("{:?}", c3);

    let s = Square {
        x: 1.0f64,
        y: 1.0f64,
        side: 12.0f64,
    };

    println!("{:?}", s);

    add(1, 2);

    let novel = String::from("call me yjh");
    let first = novel.split(' ').next().expect("could not find a `.`");
    let i = ImportantExcerpt{part: first};
    println!("{:?}", i);

    let a = 10;
    foo(a, "hello world");
}


// 在编写了很多 Rust 代码后，Rust 团队发现在特定情况下 Rust 程序员们总是重复地编写一模
// 一样的生命周期注解。这些场景是可预测的并且遵循几个明确的模式。接着 Rust 团队就把这
// 些模式编码进了 Rust 编译器中，如此借用检查器在这些情况下就能推断出生命周期而不再强
// 制程序员显式的增加注解。

// 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。这并不是
// 需要程序员遵守的规则；这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合
// 这些场景，就无需明确指定生命周期。

// 省略规则并不提供完整的推断：如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然
// 是模棱两可的话，它不会猜测剩余引用的生命周期应该是什么。在这种情况，编译器会给出
// 一个错误，这可以通过增加对应引用之间相联系的生命周期注解来解决。

// 1. 每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函
// 数有一个生命周期参数： fn foo<'a>(x: &'a i32) ，有两个引用参数的函数有两个不同的
// 生命周期参数， fn foo<'a, 'b>(x: &'a i32, y: &'b i32) ，依此类推。
// 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数： fn foo<'a>(x:
// &'a i32) -> &'a i32 。
// 3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut
// self ，那么 self 的生命周期被赋给所有输出生命周期参数。这使得方法编写起来更简
// 洁。
