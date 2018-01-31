use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::rc::Rc;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::AtomicPtr

fn main() {
    // thread_test();
    // thread_test2();
    // thread_test3();

    // channel_test();
    // channel_test2();
    // channel_test3();
    // channel_test4();
    // share_var_test();
    box_test();
}


fn thread_test() {
    // 创建一个线程
    // spawn 函数需要一个函数作为参数，且是 FnOnce 类型
    let new_thread = thread::spawn(move || {
        println!("I am a new thread.");
    });
    // 等待新建线程执行完成
    new_thread.join().unwrap();
}

// 线程执行体执行完成，线程就结束了
fn thread_test2() {
    // 创建一个线程，线程名称为 thread2, 堆栈大小为4k
    let thread2 = thread::Builder::new()
        .name("new_thread".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(move || {
            println!("I am a new thread.");
        });
    // 等待新创建的线程执行完成
    thread2.unwrap().join().unwrap();
}

// 线程结束的另一种方式就是，线程所在进程结束了
fn thread_test3() {
    let new_thread = thread::spawn(move || {
    // 再创建一个线程
    thread::spawn(move || {
        loop {
            println!("I am a new thread.");
            }
        })
    });
    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");
    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep(Duration::new(1, 0));
}

// Rust的通道( channel )可以把一个线程的消息(数据)传递到另一个线程，从而让信息在不同的线程
// 中流动，从而实现协作。通道的两端分别是发送者( Sender )和接收者( Receiver )，
// 发送者负责从一个线程发送消息，接收者则在另一个线程中接收该消息。
fn channel_test() {
    // create a channel
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // create a thread to send message
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("revice: {}", rx.recv().unwrap());
}


#[derive(Debug)]
struct Student {
    id: u32
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "student {}", self.id)
    }
}

// 消息类型必须实现 marker trait Send
// 然而由于 Send 本身是一个不安全的 marker trait ，并没有实际
// 的 API ，所以实现它很简单，但没有强制保障，就只能靠开发者自己约束，否则还是可能引发并
// 发安全问题题。对于这一点，也不必太过担心，因为Rust中已经存在的类，都已经实现
// 了 Send 或 !Send ，我们只要使用就行。

// 对于不是 Send 的情况（ !Send ），大致分为两类：
// 1. 原始指针，包括 *mut T 和 *const T ，因为不同线程通过指针都可以访问数据，从而可能引
// 发线程安全问题。
// 2. Rc 和 Weak 也不是，因为引用计数会被共享，但是并没有做并发控制。

// fn channel_test2() {
//     let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) = mpsc::channel();

//     thread::spawn(move || {
//         tx.send(Rc::new(Student{id: 1})).unwrap();
//     });

//     println!("recive: {}", rx.recv().unwrap());
// }


// 异步通道指的是：不管接收者是否正在接收消息，消息发送者在发送消息时
// 都不会阻塞。
const THREAD_COUNT: i32 = 2;

fn channel_test3() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    for id in 0..THREAD_COUNT {
        // 注意Sender是可以clone的，这样就可以支持多个发送者
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("send: {}", id + 1);
        });
    }

    thread::sleep(Duration::new(0, 10000));
    println!("wake up");
    for _ in 0..THREAD_COUNT {
        println!("recive: {}", rx.recv().unwrap());
    }
}

// 1.通道是可以同时支持多个发送者的，通过 clone 的方式来实现。 这类似于 Rc 的共享机制。 其
// 实从 Channel 所在的库名 std::sync::mpsc 也可以知道这点。 因为 mpsc 就是多生产者单消费者
// (Multiple Producers Single Consumer)的简写。 可以有多个发送者,但只能有一个接收者，即支持
// 的N:1模式。

// 2.异步通道具备消息缓存的功能，因为1和2是在没有接收之前就发了的，在此之后还能接收到这两
// 个消息。
// 那么通道到底能缓存多少消息？在理论上是无穷的, 最后的结果就是耗费内存为止。

// 3.消息发送和接收的顺序是一致的，满足先进先出原则。

// 接收者的 recv 方法应该会阻塞当前线程，如果不阻塞，在多线程的情况
// 下，发送的消息就不可能接收完全。所以没有发送者发送消息，那么接收者将会一直等待，
// 这一点要谨记。在某些场景下，一直等待是符合实际需求的。但某些情况下并不需一直等待，
// 那么就可以考虑释放通道，只要通道释放了， recv 方法就会立即返回。


// 同步通道在使用上同异步通道一样，接收端也是一样的，唯一的区别在于发送端
// 同步通道是 sync_channel ，对应的发送者也变成了 SyncSender 。
// 和异步通道相比，存在两点不同：
// 1. 同步通道是需要指定缓存的消息个数的，但需要注意的是，最小可以是0，表示没有缓存。
// 2. 发送者是会被阻塞的。当通道的缓存队列不能再缓存消息时，发送者发送消息时，就会被阻塞。
fn channel_test4() {
    // 创建一个同步通道
    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0);
    // 创建线程用于发送消息
    let new_thread = thread::spawn(move || {
    // 发送一个消息，此处是数字id
        println!("before send");
        tx.send(1).unwrap();
        println!("after send");
    });

    println!("before sleep");
    thread::sleep(Duration::new(0, 10000));
    println!("after sleep");
    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
    new_thread.join().unwrap();
}

// static如此，那const呢？ const会在编译时内联到代码中，所以不会存在某个固定的内存地址上，
// 也不存在可以修改的情况，并不是内存共享的。
static mut VAR: i32 = 0;

fn share_var_test() {
    let thread1 = thread::spawn(|| {
        unsafe {
            println!("static value: {}", VAR);
            for _ in 0..100 {
                VAR += 1;
                println!("static value: {}", VAR);            
            }
        }
    });
    unsafe {
        for _ in 0..100000 {
            VAR += 1;
        }
        println!("static value: {}", VAR);
    }
    thread1.join().unwrap();    
    unsafe {
        println!("static value: {}", VAR);
    }
}

// 由于现代操作系统的设计，线程寄生于进程，可以共享进程的资源，如果要在各个线程中共享一个
// 变量，那么除了上面的static，还有就是把变量保存在堆上了。
// 为了在堆上分配空间，Rust提供了 std::boxed::Box ，由于堆的特点，存活时间比较长，所以除
// 了我们这个地方介绍的线程间共享外，还有其他的用处
//  Box 创建的变量要想在多个线程中安全使用，我们还需要实现很多功能才行，需要是 Sync ，而 Arc 正
// 是利用 Box 来实现的一个通过引用计数来共享状态的包裹类。
fn box_test() {
    let var: Arc<i32> = Arc::new(5);
    let share_var = var.clone();
    let thread1 = thread::spawn(move || {
        println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
    });

    thread1.join().unwrap();
    println!("share value in main thread: {}, address: {:p}", var, &*var);
}

// 如果 Box 在堆上分配的资源仅在一个线程中使用，那么释放时，就非常简单，使用完，及时释放
// 即可。如果是要在多个线程中使用，就需要面临两个关键问题：
// 1. 资源何时释放？
// 2. 线程如何安全的并发修改和读取？
// 由于上面两个问题的存在，这就是为什么我们不能直接用 Box 变量在线程中共享的原因，可以看
// 出来，共享内存比消息传递机制似乎要复杂许多。Rust用了引用计数的方式来解决第一个问题，
// 在标准库中提供了两个包裹类，除了上面一个用于多线程的 std::sync::Arc 之外，还有一个不能
// 用于多线程的 std::rc::Rc 。
