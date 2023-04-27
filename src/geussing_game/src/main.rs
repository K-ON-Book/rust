use std::io;
use std::prelude::*; //prelude是每个rust程序的默认导入模块
fn main() {
    println!("猜数！");
    println!("猜测一个数");
    /* let foo =1;
    let bar =foo;
    foo=2; 没有mut关键字修饰变量，所以foo变量 immutable */
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("你猜测的数是:{}", guess)
}
