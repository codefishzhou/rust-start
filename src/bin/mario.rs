struct Mario {
    is_samll: bool,
    coins: i32,
}

impl Mario {
    fn new(&self={coins:100, is_small: true})-> Mario  {
        pub fn get_coins(&self) -> i32 {
            self.coins
        }
    }
}

fn main() {
   let marion = Mario::new();
   assert_eq!(mario.get_coins(), 100); 
}