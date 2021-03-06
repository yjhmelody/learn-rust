extern crate testing;

// 我们在顶部增加了 extern crate adder ，这在单元测试中是不需要的。这是因为每一个
// tests 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库。

// 并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)] 。Cargo 对 tests
// 文件夹特殊处理并只会在运行 cargo test 时编译这个目录中的文件。


// 我们仍然可以通过指定测试函数的名称作为 cargo test 的参数来运行特定集成测试。为了
// 运行某个特定集成测试文件中的所有测试，使用 cargo test 的 --test 后跟文件的名称
// e.g. cargo test --test integration_test
#[test]
fn test_add_two(){
    assert_eq!(4, testing::add_two(2));
}


// 将每个集成测试文件当作其自己的 crate 来对待有助于创建更类似与终端用户使用 crate 那样的单独的作用域。
// 然而，这意味着考虑到第七章学习的如何将代码分隔进模块和文件的知
// 识，tests 目录中的文件不能像 src 中的文件那样共享相同的行为。

// 对于 tests 目录中不同文件的行为，通常在如果有一系列有助于多个集成测试文件的帮助函
// 数，而你尝试遵循第七章 “将模块移动到其他文件” 部分的步骤将他们提取到一个通用的模块
// 中时显得很明显。
// 例如，如果我们创建了 tests/common.rs 并将 setup 函数放入其中，这里
// 将放入一些我们希望能够在多个测试文件的多个测试函数中调用的代码

// 为了避免 common 出现在测试输出中，不同于创建 tests/common.rs，我们将创建
// tests/common/mod.rs。在第七章的 “模块文件系统规则” 部分，对于拥有子模块的模块文件使
// 用了 module_name/mod.rs 命名规范，虽然这里 common 并没有子模块，但是这样命名告诉
// Rust 不要将 common 看作一个集成测试文件。当将 setup 代码移动到
// tests/common/mod.rs 并去掉 tests/common.rs 文件之后，测试输出中将不会出现这一部
// 分。tests 目录中的子目录不会被作为单独的 crate 编译或作为一部分出现在测试输出中。
// 一旦拥有了 tests/common/mod.rs，就可以将其作为模块来在任何集成测试文件中使用。


// 总结
// Rust 的测试功能提供了一个如何确保即使函数做出改变也能继续以期望的方式运行的途径。
// 单元测试独立的验证库的不同部分并能够测试私有实现细节。集成测试则涉及多个部分结合
// 起来工作时的用例，并像其他外部代码那样测试库的公有 API。即使 Rust 的类型系统和所有
// 权规则可以帮助避免一些 bug，不过测试对于减少代码是否符合期望相关的逻辑 bug 仍然是
// 很重要的。
