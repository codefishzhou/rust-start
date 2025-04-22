enum Message {
    Quit,
    Move { x: i32, y: i32 }, //结构体风格的变体
    // Write(String),
    ChangeColor(i32, i32, i32),
}
// - 字段有明确的名字，代码更易读懂
// - 模式匹配时可以只匹配需要的字段
// - 使用大括号，结构更清晰
fn main() {
   let msgs = [
    Message::Quit,
    Message::Move { x: 1, y: 3 },
    Message::ChangeColor(0, 160, 255),
   ];

   for msg in msgs {
      showMessage(msg);
   }
}

fn showMessage(msg: Message) {
   match msg {
      Message::Quit => println!("Quit"),
      Message::Move { x:a, y:b } => {
        assert_eq!(a,1);
        assert_eq!(b,3);
        // println!("Move to ({}, {})", x, y);
      },
      Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
   } 
}