use std::io::{self, Write};
use std::thread;
use std::time::Duration;
fn main() {
    let mut bar = String::new();
    for i in 1..=100 {
        if i % 4 == 0 {
            bar = bar + "=";
        }
        let bar = format!("{:25}", bar);
        thread::sleep(Duration::from_secs_f64(0.05));
        print!("\r[{}] {}%", bar, i); // print! 需要显式地调用 io::stdout().flush() 方法来手动刷新输出。println! 会自动调用;
        io::stdout().flush().unwrap();
    }
}
