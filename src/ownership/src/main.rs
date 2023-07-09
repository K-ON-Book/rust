fn main() {
    /* let mut s = String::from("hello"); // String
    s.push_str(", world");
    println!("{}", s);
    let b = "sasas"; // &str
    println!("{}", b); */
    //

    /* let s1 = String::from("Hello");
    let s2 = s1; //赋值给s2后s1失效
    println!("{}", s1); // borrow of moved value */

    /* let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1:{:p} s2:{:p}", s1.as_ptr(), s2.as_ptr()); */

    let s = String::from("Hello World");
    take_ownership(s);
    let x = 5;
    makes_copy(x);

    println!("x: {}", x);
    // println!("s: {}", s); // borrow of moved value

    /* let s1 = gives_ownership();
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3); */

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

#[allow(dead_code)]
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
#[allow(dead_code)]
fn makes_copy(some_number: i32) {
    println!("some_number: {}", some_number + 1);
}

#[allow(dead_code)]
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

#[allow(dead_code)]
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

#[allow(dead_code)]
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
