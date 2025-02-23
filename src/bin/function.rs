fn main(){
    let a = plus_one(5);
    println!("a: {}", a);
}
/**
 * 函数定义
 * fn 函数名(参数名:参数类型) -> 返回类型{
 *     函数体
 * }
 */
fn plus_one(x:i32) -> i32{
    x + 1
}