use std::io;
use rand::Rng;

use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // 在 rand 0.8 及以上版本中，使用 ..= 表示闭区间（包含边界值）
    // thread_rng 创建一个线程本地的随机数生成器（ThreadRng）。
    let secret_number = rand::rng().random_range(1..=100);
    let guess = String::new(); //创建一个mut的变量为一个null的string
    let mut guess_number = String::new(); //创建一个mut的变量为一个null的string

    io::stdin()
        .read_line(&mut guess_number)
        .expect("Failed to read line"); //不写错误处理,  会出现警告
    // &mut guess as the argument to read_line to tell it what string to store the user input in.
    println!("You guessed: {}", guess_number);
    println!("You random number: {}", secret_number);
    // 将字符串转换为数字
    let guess_number: u32 = guess_number.trim().parse().expect("Please type a number!");
    match guess_number.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

//use std::io; 之后可以