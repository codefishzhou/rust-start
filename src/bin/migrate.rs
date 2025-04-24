// trait 关键字 + MigrateBird 特征名


// 定义大雁结构体
struct WildGoose {
     color : String,
}

struct swallow {
    color : String,
}

trait Tweet {
    fn tweet(&self) -> String;
}

impl Tweet for WildGoose {
    fn tweet(&self) -> String {
        "ga ga".to_string()
    }
}

impl MigrateBird for swallow {
    fn tweet(&self) -> String {
        "ji ji zha zha".to_string()
    }
}

