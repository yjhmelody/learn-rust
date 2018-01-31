

fn main() {
    unsafe_test();
    ptr_test();
    into_raw_test();
}

// unsafe fn不安全函数标示如果调用它可能会违反Rust的内存安全语意
unsafe fn foo(ptr: *mut i32) {
    println!("unsafe: {}", *ptr);
    *ptr += 1;
}

fn unsafe_test() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe{ *raw };
    println!("raw point at {}", points_at);

    static mut N: i32 = 5;
    // unsafe block不安全块可以在其中调用不安全的代码
    unsafe {
        N += 1;
        println!("N: {}", N);
        foo(&mut N as *mut i32);
        println!("N: {}", N);        
    }
}

// unsafe trait不安全trait及它们的实现，所有实现它们的具体类型有可能是不安全的
unsafe trait Scary {
    // add code here
}
unsafe impl Scary for i32 {
    // add code here
}

// Rust通过限制智能指针的行为保障了编译时安全，不过仍需要对指针做一些额外的操作。
// *const T和*mut T在Rust中被称为“裸指针”。它允许别名，允许用来写共享所有权的类型，
// 甚至是内存安全的共享内存类型如：Rc<T>和Arc<T>，但是赋予你更多权利的同时意味着你需要担当更多的责任:
// 不能保证指向有效的内存，甚至不能保证是非空的
// 没有任何自动清除，所以需要手动管理资源
// 是普通旧式类型，也就是说，它不移动所有权，因此Rust编译器不能保证不出像释放后使用这种bug
// 缺少任何形式的生命周期，不像&，因此编译器不能判断出悬垂指针
// 除了不允许直接通过*const T改变外，没有别名或可变性的保障

fn ptr_test() {
    let a = 1;
    let b = &a as *const i32;

    let mut x = 2;
    let y = &mut x as *mut i32;
    // 解引用需要在unsafe中进行
    let c = unsafe { *b };
    println!("c: {}", c);
}


fn into_raw_test() {
    let a: Box<i32> = Box::new(10);
    // 我们需要先解引用a，再隐式把 & 转换成 *
    let b: *const i32 = &*a;
    // 引用和裸指针之间可以隐式转换，但隐式转换后再解引用需要使用unsafe    
    let c: *const i32 = Box::into_raw(a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}