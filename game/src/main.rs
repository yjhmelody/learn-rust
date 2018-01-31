extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Fail to read line");

        // Rust 允许 隐藏（shadow），用一个新值来隐藏 guess 之前的值
        // 将 expect 调用换成 match 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法。须
        // 知 parse 返回一个 Result 类型，而 Result 是一个拥有 Ok 或 Err 成员的枚举。这里
        // 使用的 match 表达式，和之前处理 cmp 方法返回 Ordering 时用的一样。
        // 如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok 。这个
        // Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num ，
        // 最后如愿变成新创建的 guess 变量。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //  _ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
