### 基础概念
cargo 是rust的包管理工具
rustc 是rust的编译器
rustdoc 是rust的文档生成工具
rustfmt 是rust的代码格式化工具
rust-analyzer 是rust的智能补全工具

#### cargo 
cargo 是rust的包管理工具
cargo build 编译项目
cargo run 运行项目
cargo check 检查项目
cargo fmt 格式化代码
cargo clippy 检查代码
cargo doc 生成文档
cargo build --release 编译项目（优化）
cargo clean 清理项目
cargo update 更新依赖
cargo add 添加依赖
cargo remove 删除依赖


### 1. 简单的hello

```rust
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = "Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
```

`
cargo run 
`
#运行src/main.rs

也可以手动执行特定文件
`cargo run -- --src main1.rs`
或者
`cargo run ./src/main1.rs`

--bin 指定运行哪个二进制文件



std::io::{stdout, BufWriter};
std::io 为rust自带的标准库

提供 
1. 标准输入输出：如 stdin()、stdout()、stderr()
文件操作：如 File 类型用于读写文件
3. 缓冲读写：如 BufReader、BufWriter
错误处理：如 Error 类型
其他工具：如 Cursor、Seek 等

```rust
   let name = "Rust";
     println!("Hello, {}!", name);
```
println! 是rust的标准库提供的宏，用于打印信息到控制台
println! 后面的感叹号表示它是一个宏，用于格式化输出。宏是 Rust 中非常强大的工具，可以处理比普通函数更复杂的逻辑。


### 2. 第一个关键点, 所以权系统
```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");1

    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    
    let mut guess = String::new(); //创建一个mut的变量为一个null的string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // &mut guess as the argument to read_line to tell it what string to store the user input in.
    println!("You guessed: {}", guess);
}

```
关键内容在`&mut guess` 这是一个对guess的引用，& 表示引用，mut 表示可变引用。, 如果传递的是不可变引用(&guess)，则不能修改guess的值。, 

**Rust 的所有权系统确保内存安全。通过传递可变引用（&mut guess），read_line 可以借用 guess 并修改它，而不会获取 guess 的所有权。
这样，guess 的所有权仍然在 main 函数中，read_line 只是临时借用它。**

声明数字类型
`let guess_number: u32 = 42;`

