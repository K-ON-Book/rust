fn main() {
    if {
        let num = 3;
        num < 5
    } {
        println!("condition is true");
    } else {
        println!("condition is flase");
    }

    let num = 6;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3 or 2")
    }
    // 上述 else if过多，采用match 重构
    let mut collect: Vec<i32> = vec![];
    for i in (0..num).rev() {
        if i > 0 {
            match num % i {
                0 => {
                    collect.push(i);
                }
                _ => {
                    // println!("num is not divisible by {}", i)
                }
            }
        } else {
            println!("number is divisible by {:?}", collect);
        }
    }

    // if是表达式，可以赋值给变量
    let condition = true;
    let value = if condition { '5' } else { '3' }; // if else 块里面的值类型必须相同
    println!("'{}'", value);
}
