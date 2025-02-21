use std::io;

fn main(){
    print_fn()
}

fn handle_operation(code: &str, num1: i32, num2: i32) -> i32 {
    println!("code:{}", code);
    match code.trim() {
        "+" => add(num1, num2),
        "-" => sub(num1, num2),
        "*" => mul(num1, num2),
        "/" => div(num1, num2),
        _ => 0,
    }

}

fn add(a: i32, b:i32) -> i32 {
    // Rust 中函数的返回值可以是最后一个表达式（不加分号）
    a + b
}

fn sub(a: i32, b:i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn judgeCode(code: &str) -> bool {
    match code.trim() {
        "=" => true, // 匹配任意一个
        _ => false,
    }
}

fn print_fn() {
    let mut pause = String::from("+");
    let mut switch_flag = false;
    let mut mul_number = String::new();
    let mut result = 0;
    
    while pause.trim() != "=" {
        println!("pause: {}", pause);
        match switch_flag {
            true => {
                println!("now need input a mul code(+-*/):");
                pause.clear(); 
                io::stdin().read_line(&mut pause).expect("读取失败");
                switch_flag = false;
            },
            false => {
                println!("now need input a number:");
                io::stdin().read_line(&mut mul_number).expect("读取失败");
                
                let number: i32 = mul_number.trim().parse().expect("请输入数字");
                
                result = handle_operation(&pause, result, number);
                
                switch_flag = true;
                mul_number.clear();
            },
        }
    }
    println!("最终结果: {}", result);
}
