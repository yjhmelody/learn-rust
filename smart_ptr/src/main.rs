use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::sync::RwLock;
use std::cell::Cell;
use std::cell::RefCell;

use std::collections::HashMap;


fn main() {
    // rc_test();
    // weak_test();
    // arc_test();
    // mutex_test();
    // rwLock_test();
    // cell_test();
    ref_cell_test();
}


// Rc 用于同一线程内部，通过 use std::rc::Rc 来引入。它有以下几个特点：
// 1. 用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的
// T 对象，只能读；
// 2. 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
// 3. Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
// 4. Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值
// 这一说）。
fn rc_test() {
    let five = Rc::new(5);
    let five2 = five.clone();
    let five3 = five.clone();
    println!("{} {} {}", five, five2, five3);
}


// Rc 是一个引用计数指针，而 Weak 是一个指针，但不增加引用计数，是 Rc 的 weak 版。它有
// 以下几个特点：
// 1. 可访问，但不拥有。不增加引用计数，因此，不会对资源回收管理造成影响；
// 2. 可由 Rc<T> 调用 downgrade 方法而转换成 Weak<T> ；
// 3. Weak<T> 可以使用 upgrade 方法转换成 Option<Rc<T>> ，如果资源已经被释放，则 Option
// 值为 None ；
// 4. 常用于解决循环引用的问题。
fn weak_test() {
    let five = Rc::new(5i32);
    let weak_five = Rc::downgrade(&five);
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("{} {:?} {:?}", five, weak_five, strong_five);
}



// Arc 是原子引用计数，是 Rc 的多线程版本。 Arc 通过 std::sync::Arc 引入。
// 它的特点：
// 1. Arc 可跨线程传递，用于跨线程共享一个对象；
// 2. 用 Arc 包裹起来的类型对象，对可变性没有要求；
// 3. 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
// 4. Arc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值
// 这一说）；
// 5. Arc 对于多线程的共享状态几乎是必须的（减少复制，提高性能）。

// 与 Rc 类似， Arc 也有一个对应的 Weak 类型，从 std::sync::Weak 引入。
// 意义与用法与 Rc Weak 基本一致，不同的点是这是多线程的版本
fn arc_test() {
    let numbers: Vec<_> = (0..5u32).collect();
    let shared_numbers = Arc::new(numbers);

    for _ in 0..3 {
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &child_numbers[..];
            // work wtih local numbers
            println!("{:?}", local_numbers);
        });
    }
    thread::park_timeout(Duration::new(1, 0));
}


fn mutex_test() {
    const N:usize = 8;

    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();

    for i in 0..10 {
        println!("{}", i);
        let(data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
}

fn rwLock_test() {
    let lock = RwLock::new(5);
    // many reader locks can be held at once
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("reader: {}", *r1);
        println!("reader: {}", *r2);
        // read locks are dropped at this point
    }

    // only one write lock may be held, however
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("writer: {}", *w);
         // write lock is dropped here
    }
}


// Cell , RefCell ，它们弥补了 Rust 所有权机制在灵活性上和某些场景下的不足。
// 同时，又没有打破 Rust 的核心设计。它们的出现，使得Rust 革命性的语言理论设计更加完整，更加实用。
// 具体是因为，它们提供了 内部可变性 （相对于标准的 继承可变性 来讲的）。
// 通常，我们要修改一个对象，必须
// 1. 成为它的拥有者，并且声明 mut ；
// 2. 或 以 &mut 的形式，借用；
// 而通过 Cell , RefCell ，我们可以在需要的时候，就可以修改里面的对象。而不受编译期静态借用规则束缚。
fn cell_test() {
    let c = Cell::new(5);
    let five = c.get();
    c.set(6);
    println!("cell: {:?} get: {}", c, five);
}

// 相对于 Cell 只能包裹实现了 Copy 的类型， RefCell 用于更普遍的情况（其它情况都用
// RefCell ）
// 相对于标准情况的 静态借用 ， RefCell 实现了 运行时借用 ，这个借用是临时的。这意味着，编
// 译器对 RefCell 中的内容，不会做静态借用检查，也意味着，出了什么问题，用户自己负责。
// RefCell 的特点：
// 1. 在不确定一个对象是否实现了 Copy 时，直接选 RefCell ；
// 2. 如果被包裹对象，同时被可变借用了两次，则会导致线程崩溃。所以需要用户自行判断；
// 3. RefCell 只能用于线程内部，不能跨线程；
// 4. RefCell 常常与 Rc 配合使用（都是单线程内部使用）；
fn ref_cell_test() {
    let shared_map = Rc::new(RefCell::new(HashMap::<&'static str, i32>::new()));
    shared_map.borrow_mut().insert("1", 1);
    shared_map.borrow_mut().insert("2", 2);
    shared_map.borrow_mut().insert("3", 3);    

    println!("{:?}", shared_map.borrow());
}