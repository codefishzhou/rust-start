use std::collections::HashMap;
/**
 * iter() 返回的是一个迭代器，它可以用来遍历 HashMap 中的元素。是不可变引用
 * iter_mut() 返回的是一个迭代器，它可以用来遍历 HashMap 中的元素。是可变引用
*/
fn main(){ 
    let map:HashMap<&str, i32> = HashMap::new();
    let student_arr :[(&str, i32); 3] = [
        ("Alice", 20),
        ("Bob", 21),
        ("Charlie", 22),
    ];
    let mut student_map = HashMap::new();
    for (key, value) in student_arr {
        student_map.insert(key, value);
    }
    let alice_grade = student_map.entry("Alice").or_insert(10);
    println!("{:?}", *alice_grade);
    println!("{:?}", assert_eq!(*alice_grade, 20));
}


