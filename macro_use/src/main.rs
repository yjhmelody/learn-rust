// 另外从形式上看，与函数调用的另一个区别是参数可以用圆括号( () )、花括号( {} )、方括号( [] )中的任意一种括起来，比如这行也可
// 以写成 println!["Hello, world!"] 或 println!{"Hello, world!"} ，不过对于 Rust 内置的宏都
// 有约定俗成的括号，比如 vec! 用方括号， assert_eq! 用圆括号

// 首先 Rust 的函数不能接受任意多个参数，其次函数是不能操作语法单元的，即把语法元素作为参数进行操作，从而生成代码
// 相比函数，宏是用来生成代码的，在调用宏的地方，编译器会先将宏进行展开，生成代码，然后再编译展开后的代码。

// 宏定义格式是： macro_rules! macro_name { macro_body } ，其中 macro_body 与模式匹配很像， 
// pattern => do_something ，所以 Rust 的宏又称为 Macro by example (基于例子的宏)
// 其中 pattern 和 do_something 都是用配对的括号括起来的，括号可以是圆括号、方括号、花括号
// 中的任意一种。匹配可以有多个分支，每个分支以分号结束。
// 对每一条模式按顺序进行匹配，只要有一个匹配上，就会将 => 左边定义的参数代入右边进行替换，
// 如果替换不成功，编译器就会报错而不会往下继续匹配，替换成功就会将右边替换后的代码放在宏调用的地方。

// 宏定义里面的变量都是以 $ 开头的，相应的类型也是以冒号分隔说明，这里
// ident 是变量 $func_name 的类型，表示这个变量是一个 identifier ，这是语法层面的类型
// (designator)，而普通的类型如 char, &str, i32, f64 这些是语义层面的类型。

// 卫生宏最开始是由 Scheme 语言引入的，后来好多语言基本都采用卫生宏，即编译器或运行时会
// 保证宏里面定义的变量或函数不会与外面的冲突，在宏里面以普通方式定义的变量作用域不会跑到宏外面。

macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name));
        }
    )
}

macro_rules! bar {
    ($x:ident) => { println!("The argument you passed to macro is {}", $x); }
}

fn main() {
    create_function!(foo);
    foo();
    let x = 1;
    bar!(x);
}




// 所以最后编译器编译的实际代码是
// fn main() {
//     fn foo() {
//         println!("function {:?} is called", stringify!(foo))
//     }
//     foo();
// }
