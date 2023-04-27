//prelude是每个rust程序的默认导入模块
use rand::Rng; // trait
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜数！");
    println!("猜测一个数");
    let secret_number = rand::thread_rng().gen_range(1..101); //左闭右开
    println!("神秘数字:{}", secret_number);
    /* let foo =1;
    let bar =foo;
    foo=2; 没有mut关键字修饰变量，所以foo变量 immutable */
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        let guess: u32 = match guess.trim().parse() {
            // expect("Please type a number!");
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是:{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
