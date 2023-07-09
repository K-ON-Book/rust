fn main() {
    learn_loop();
    learn_while();
    learn_for();
}
fn learn_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result: {}", result)
}
fn learn_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!");
}
fn learn_for() {
    let arr: [i32; 5] = [10; 5];
    for element in arr.iter() {
        println!("{}", element);
    }

    //实现上面的 while 实现的功能
    for number in (1..=3).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF");
}
