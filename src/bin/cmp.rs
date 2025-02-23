use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main(){



    // match number.cmp(&random_num){
    //     Ordering::Less => println!("too small"),
    //     Ordering::Greater => println!("too big"),
    //     Ordering::Equal => println!("you win"),
    // }
    
    loop {
        println!("gugess a number: (0-10)");
        let random_num = rand::thread_rng().gen_range(0..=10);
        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num).expect("failed to read line");
        let number:i32 =match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字！");
                continue;
            }
        };

        match number.cmp(&random_num){
            Ordering::Less => {
                println!("too small: {}", random_num);
            },
            Ordering::Greater => {
                println!("too big: {}", random_num);
            },
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }

}

// 常见错误 stdin => stdio
// 类型转换  
// &mut 和 & 的区别 后者为引用