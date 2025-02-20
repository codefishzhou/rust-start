fn main(){
    correct_fn();
    // MixTableEg();
}

fn mix_table_eg(){
    let mut value = 1;
    // let newValue = &mut value;
    // value = 2;
    // println!("value: {}, newValue: {}", value, newValue);
}

fn correct_fn(){
    let mut value = 1;
    let mut_value = &mut value;
    *mut_value = 2; //解引用操作符号
    println!("mut_value: {}", mut_value);
  }

  fn mut_table_eg(){
    let mut value = 1;
    {
        let mut_value = &mut value;
        *mut_value = 2; //解引用操作符号
    }
    println!("value: {}", value);
  }
  