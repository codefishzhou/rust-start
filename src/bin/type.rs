fn main(){
    let bool:bool = true;          // 布尔类型，true或false
    let int:i32 = 1;               // 32位有符号整数
    let float:f32 = 1.0;           // 32位浮点数
    let str:String = "hello".to_string(); // String类型（堆分配的UTF-8字符串）
    let char:char = 'h';           // Unicode标量值（4字节）
    let array:[i32;5] = [1,2,3,4,5]; // 固定长度数组（i32类型，5个元素）
    let tuple:(i32,f32,char) = (1,1.0,'h'); // 元组（可包含不同类型）
    let // 不可变切片（数组的视图）
    let mut_slice:&mut [i32] = &mut array[0..2]; // 可变切片（需要数组可变）
    // let mut_slice_length:usize = mut_slice.len();
    // let mut_slice_capacity:usize = mut_slice.capacity();
    let b = 4;
    let a =  6;
    println!("a: {}", a);
    println!("b: {}", b);
}