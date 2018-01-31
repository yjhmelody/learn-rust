use std::env;

fn main(){
        
    let file_name = "foobar.rs";

    match extension_explicit(file_name) {
        None => println!("None"),
        Some(name) => println!("name: {}", name),
    }


    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}



fn guess (n: i32) -> bool {
    // panic 会导致当前线程结束，甚至是整个程序的结束，这往往是不被期望看到的结果
    if n < 1 || n > 10 {
        panic!("Invaid number");
    }
    n == 5
}


// Option 是Rust的系统类型，用来表示值不存在的可能，
// 这在编程中是一个好的实践，它强制Rust检测和处理值不存在的情况
// Rust 使用模式匹配来处理返回值，调用者必须处理结果为 None 的情况。这往往是一个好的编程
// 习惯，可以减少潜在的bug。Option 包含一些方法来简化模式匹配
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c ) in haystack.char_indices(){
        if c == needle {
            return Some(offset);
        }
    }
    None
}


fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..])
    }
}


fn map<F, T, A>(option: Option<T>, f:F)-> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

// type Option<T> = Result<T, ()>;


// 可以在值为 None 的时候返回一个 Result::Err(E) ，值为 Some(T) 的时候返回 Ok(T) ，
// 利用它我们可以组合 Option 和 Result
fn ok_or<T, E> (option: Option<T>, err:E) -> Result<T, E> {
    match option {
        Some(value) => Ok(value),
        None => Err(err),
    }
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

