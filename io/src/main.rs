// use std::io;
// use std::i32;

use std::io;
use std::env;
use io::Read;
use io::Write;
use io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

// 标准库里面把怎么读和怎么写抽象出来归到了 Read 和 Write 两个接口里面，实现了 Read 接口的叫 reader，而实现了
// Write 的叫 writer。Rust里面的 Trait 比其它语言里面的接口更好的一个地方是 Trait 可以带默认
// 实现，比如用户定义的 reader 只需要实现 read 一个方法就可以调用 Read trait 里面的任意其
// 它方法，而 writer 也只需要实现 write 和 flush 两个方法。

// 如果给 reader增加一个 buffer，在调用时 read 方法时多读一些数据放在 buffer 里面，
// 下次调用 read 方法时就有可能只需要从 buffer 里面取数据而不用调用系统API了，
// 从而减少了系统调用次数提高了读取效率，这就是所谓的 BufRead Trait

fn main() {
    // let mut string = String::from("Hello world");
    // read_from_stdin(&mut string);
    
    // let arr:[u8; 3] = [64, 65, 66];
    // write_to_stdout(&arr);

    // get_number();

    // let args = env::args();
    // for (i, arg) in args.enumerate() {
    //     println!("arg{}: {}", i, arg);
    // }

    // let content = String::from("test我的世界");
    // create_file("test.md", content.as_bytes());
    // let mut content = String::new();
    // read_file("test.md", &mut content);
    // println!("read file: {}", content);

    // let file = OpenOptions::new().write(true).truncate(true).open("test.md");

    let b = foo("world");
    println!("{}", b);

}

// Rust 中是在编译期编译器借助 lifetime 对堆内存生命期进行分析，在生命期结束
// 时自动插入 free 。当前 Rust 底层即 Box 背后是调用 jemalloc 来做内存管理的，所以堆上空间
// 是不需要程序员手动去管理释放的

fn foo(x: &str) -> String {
    let a = "Hello 我".to_string() + x;
    a
}

fn create_file(filename: &str, buf:&[u8]) -> io::Result<()> {
    let mut f = try!(File::create(filename));
    try!(f.write(&buf));
    return Ok(());
}


fn read_file(filename: &str, buf: &mut String) -> io::Result<()> {
    let mut f = try!(File::open(filename));
    try!(f.read_to_string(buf));
    Ok(())
}



fn read_from_stdin(buf: &mut String)-> io::Result<()> {
    // try! 在遇到错误时会把错误 return 出去的，所以需要保证包含 try! 语句的函数其返回类型是 io::Result<T>
    try!(io::stdin().read_line(buf));
    Ok(())
}

fn write_to_stdout(buf: &[u8])-> io::Result<()> {
    try!(io::stdout().write(&buf));
    Ok(())
}

fn get_number() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line");

    // 这里等效的写法是：
    // let num: i32 = input.trim().parse().unwrap();
    let num = input.trim().parse::<i32>().unwrap();
    println!("您输入的数字是：{}", num);
}

fn say_hello(name: &str) {
    println!("hello {}", name);
}

fn say_hello2(name: String) {
    println!("hello {}", name);
}

fn print() {
    println!("It is {0} that {1} is {0}", 0, 1);
    
    println!("{:.2}", 1.234);

    println!("B: {:b} H: {:x} 0: {:o}", 10, 10, 10);

    let (f_name, l_name) = ("jh", "y");
    
    println!("{}{}", l_name, f_name);

    println!("{ten:>0ws$}", ten=10, ws=5);

    let neg_4 = -4i32;

    println!("{}", neg_4);

    println!("{}", neg_4.abs());

    println!("max: {}", 4f64.max(5f64));

    println!("sin 3.14: {}", 3.14f64.sin());


    let age = 6;
    if age == 5 {
        println!("Go to Kindergarten");
    }else if age > 5 && age < 18 {
        println!("go to grade {}", age - 5);
    }else if age <= 25 {
        println!("go to college");
    }else {
        println!("do what you want");
    }

    println!("!true = {}", !true);
    println!("true || false = {}", true || false);

    let can_vote = if age >= 18 {
        true
    } else {
        false
    };

    println!("Can vote: {}", can_vote);

    let mut x = 1;

    loop {
        if x % 2 == 0 {
            println!("{}", x);
            x += 1;
            continue;            
        }

        if x > 10 {
            break;
        }
        x += 1;
    }
    
    let mut y = 1;
    while y <= 5 {
        println!("{}", y);
        y += 1;
    }

    for z in 1..6 {
        println!("z: {}", z);
    }

    let rand_string = "I am a random string";
    println!("Length: {}", rand_string.len());

    let (first, second) = rand_string.split_at(6);
    println!("{}     {}", first, second);

    let mut chars = rand_string.chars();

    for ch in &mut chars {
        println!("{}", ch);
    }

    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }

        indiv_char = chars.next();
    }

    let mut vect1 = vec![1,2,3,4,5];

    for i in &vect1 {
        println!("Vect: {}", i);
    }

    vect1.push(6);

    for i in &vect1 {
        println!("Vect: {}", i);
    }

    let tuple = ("Derek", 40);
    let tuple2: (&str, i8) = ("Derek", 40);
    println!("{} {}", tuple.0, tuple.1);
    println!("{} {}", tuple2.0, tuple2.1);

    say_hello("yjh");
    let mut name = String::new();
    name.push_str("yjh");
    say_hello2(name);
}
