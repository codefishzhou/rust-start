fn main(){
    let mut x = 5;
    let y = 5;
    const HOUR:u32 = 60 * 60 * 60 ;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    println!("{}", y);
    println!("{}", HOUR);

    let str = "     ";
    let mut mut_str = str.clone();
    // mut_str = mut_str.len();
    let mut_str_length :u32 = mut_str.len().try_into().unwrap();
    println!("{}, length: {}", str, str.len());
    println!("mut_str, {}", mut_str_length);

}