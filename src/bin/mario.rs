struct Mario {
    is_small: bool,
    coins: i32,
}

impl Mario {
    fn new() -> Self {
        Mario {
            is_small : false ,
            coins : 100 ,
        }
    }

    fn get_coins(&self) -> i32 {
        self.coins
    }
}

fn main() {
   let mario = Mario::new();
   assert_eq!(mario.get_coins(), 100); 
}