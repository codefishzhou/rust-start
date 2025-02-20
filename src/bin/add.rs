use std::io;

fn main(){
    let mut operation_number1 = String::new();
    let mut operation_number2 = String::new();
    let mut operation_code = String::new();
    println!("print a number:");
    io::stdin()
        .read_line(&mut operation_number1)
        .expect("Failed to read line");
    println!("print a operation code, +-*/");
    io::stdin().read_line(&mut operation_code).expect("Failed to read line");
    println!("now is {} {} ", &operation_number1, &operation_code);
    io::stdin().read_line(&mut operation_number2).expect("Failed to read line");
    println!("now is {} {} {} = {}", &operation_number1, &operation_code, &operation_number2, judge_operation(&operation_code, &operation_number1, &operation_number2));
}

fn judge_operation(code: &str, a: &str, b: &str) -> i32 {
    let num1 = a.trim().parse::<i32>().unwrap();
    let num2 = b.trim().parse::<i32>().unwrap();
    println!("code:{}", code);
    match code.trim() {
        "+" => add(num1, num2),
        "-" => sub(num1, num2),
        _ => 0,
    }

}

fn add(a: i32, b:i32) -> i32 {
    println!("a:{}, b:{}, =:{}", a,b, a + b);
    // Rust 中函数的返回值可以是最后一个表达式（不加分号）
    a + b
}

fn sub(a: i32, b:i32) -> i32 {
    a - b
}
