
// 如果测试通过了，Rust 的测试库默认会捕获打印到标准输出的任何内容

#[cfg(test)]
mod tests {
    use super::*;
    // tests 是一个普通的模块，它遵循第
    // 七章 “私有性规则” 部分介绍的常用可见性规则。因为这是一个内部模块，需要将外部模块中
    // 被测试的代码引入到内部模块的作用域中。这里选择使用全局导入使得外部模块定义的所有
    // 内容在 tests 模块中都是可用的。

    // 自定义错误信息
    // 也可以向 assert! 、 assert_eq! 和 assert_ne! 宏传递一个可选的参数来增加用于打印的自
    // 定义错误信息。


    // 有时一些特定的测试执行起来是非常耗费时间的，所以在大多数运行 cargo test 的时候希
    // 望能排除他们。与其通过参数列举出所有希望运行的测试，也可以使用 ignore 属性来标记
    // 耗时的测试并排除他们
    
    #[test]
    #[ignore]
    // 对想要排除的测试的 #[test] 之后增加了 #[ignore] 行。
    // 如果只希望运行被忽略的测试，可以使用 cargo test -- --ignored
    fn expploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // 可以使用失败测试的名称来只运行这个测试，这样比较方便调试
    // fn will_fail() {
        // panic!("Make this test fail");
    // }

    // assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。需要向
    // assert! 宏提供一个计算为布尔值的参数。如果值是 true ， assert! 什么也不做同时测试
    // 会通过。如果值为 false ， assert! 调用 panic! 宏，这会导致测试失败。 assert! 宏帮
    // 助我们检查代码是否以期望的方式运行。
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{length: 8, width: 7};
        let smaller = Rectangle{length: 5, width: 4};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{length: 8, width: 7};
        let smaller = Rectangle{length: 5, width: 4};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(1), 3);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Tommy");
        // assert!(result.contains("yjh"), "Greeting did not contain name, value was `{}`", result);
        assert!(result.contains("Tommy"), "Greeting did not contain name, value was `{}`", result);
        
    }

    // 除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误情况也是很重要的。
    // 然而 should_panic 测试可能是非常含糊不清的，因为他们只是告诉我们代码并没有产生
    // panic。 should_panic 甚至在测试因为其他不同的原因而不是我们期望发生的情况而 panic 时
    // 也会通过。为了使 should_panic 测试更精确，可以给 should_panic 属性增加一个可选的
    // expected 参数。测试工具会确保错误信息中包含其提供的文本。

    #[test]
    #[should_panic(excepted = "Guess value must be less than or equal to 100")]
    fn test_guess() {
        Guess::new(200);
        println!("test_guess\n\n");
    }
}


#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!",name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

// 单元测试

// 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的
// 定位代码位于何处和是否符合预期。单元测试位于 src 目录中，与他们要测试的代码存在于相
// 同的文件中。传统做法是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test)标注模块。

// 测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，
// 而在运行 cargo build 时不这么做。这在只希望构建库的时候可以节省编译时间，并能节省
// 编译产物的空间因为他们并没有包含测试。

// 因为单元测试位于与源码相同的文件中，所以使用#[cfg(test)] 来指定他们不应该被包含进编译结果中
#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_inner() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}

// Rust 的私有性规则确实允许你测试私有函数，由于私有性规则。

// 注意 internal_adder 函数并没有标记为 pub ，不过因为测试也不过是 Rust 代码同时
// tests 也仅仅是另一个模块，我们完全可以在测试中导入和调用 internal_adder 。如果你并
// 不认为私有函数应该被测试，Rust 也不会强迫你这么做。
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// 集成测试

// 在 Rust 中，集成测试对于需要测试的库来完全说是外部的。他们同其他代码一样使用库文
// 件，这意味着他们只能调用作为库公有 API 的一部分函数。他们的目的是测试库的多个部分
// 能否一起正常工作。每个能单独正确运行的代码单元集成在一起也可能会出现问题，所以集
// 成测试的覆盖率也是很重要的。

// 为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。Cargo 知道如何去
// 寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo
// 会将每一个文件当作单独的 crate 来编译。

