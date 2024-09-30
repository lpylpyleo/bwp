use std::io;
use std::str::FromStr;

pub fn require_user_input<T>() -> T
where
    T: FromStr,              // T 必须实现 FromStr trait
    T::Err: std::fmt::Debug, // FromStr 解析错误需要实现 Debug
{
    loop {
        let mut input = String::new();

        // 从标准输入获取用户输入
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 尝试将输入解析为 T 类型
        match input.trim().parse::<T>() {
            Ok(value) => return value, // 如果成功解析，返回 T 类型值
            Err(_) => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}
