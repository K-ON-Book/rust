fn main() {
    another_function1();
    another_function2(5, 7);

    let x = 5;
    let y = {
        let x = 1;
        x + 3
    };
    println!("{}", y);
    let z = { x + 1 };
    println!("{}", z);
    let m = 0;
    let _ = m + 1;

    println!("{}", five(2))
}

fn another_function1() {
    println!("Another function1");
}
fn another_function2(x: i32, y: u32) -> (i32, u32) {
    println!("x: {}, y: {}", x, y);
    (x, y)
}

///函数的返回值
/// - 在 -> 后面声明函数返回值的类型，不可以为返回值命名
/// - 在Rust里面，返回值就是函数体里面最后一个表达式的值
/// - 如想提前返回，需要使用 return 关键字，并指定值
fn five(x: i32) -> i32 {
    x + 5
}
