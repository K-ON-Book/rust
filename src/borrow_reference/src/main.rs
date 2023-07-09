fn main() {
    /* let mys = String::from("hello");
    let len = calculate_length(&mys);
    println!("'{}' len: {}", mys, len); */

    /* let mut s1 = String::from("hello");
    modify_s(&mut s1);
    println!("s1: '{}'", s1); */

    /* let s = String::from("hello");
    let s1 = &s;
    println!("{:p} {:p}", s.as_ptr(), s1.as_ptr());

    let s = String::from("hello");
    let s1 = s.clone();
    println!("{:p} {:p}", s.as_ptr(), s1.as_ptr());

    let n1 = 5;
    let mut n2 = n1;
    println!("{:p} {:p}", n1 as *const i32, n2 as *const i32); //取得相同的地址，是因为编译器做了优化，因为他们存储的值都是5,所以将其存储在相同内存位置，提高性能和节省内存
    n2 = 10;
    println!("after overwritting n2 value,the ptr {:p}", n2 as *const i32); */

    /* let mut s = String::from("hello");
    {
        //同一作用域，不能同时出现多个可变引用
        let s1 = &mut s;
        println!("{}", s1);
    }
    let s3 = &mut s;
    println!("{}", s3); */

    /* let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    let s1 = &mut s;
    println!("{}", s1);
    let s2 = &mut s;
    let s3 = &s;
    println!("{}{}", s2, s3); */

    /* let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let s1 = &mut s; //多个不可变引用是允许的，但是不能同时出现可变和不可变引用
    println!("{}{}{}", r1, r2, s1) */

    /* //悬空引用
    let s = dangle(); */
}

/// ## 总结
/// 引用的规制：
/// - 在任何给定的时刻，只能满足以下条件之一：
/// - - 一个可变的引用
/// - - 任意数量的不可变引用
///
/// - 引用必须一直有效

fn dangle() -> &String {
    let s = String::from("hello");
    &s //不能引用离开自己作用域就销毁的s
}

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn modify_s(s: &mut String) {
    s.push_str(", world")
}
