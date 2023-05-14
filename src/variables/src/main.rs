fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    //数据类型推断
    let guess: i32 = "42".parse().expect("Not a num");
    println!("{}", guess);
}
