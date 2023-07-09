fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    //数据类型推断
    let guess = "42".parse().expect("Not a num");
    println!("{}", guess);
    let test_guess: u32 = guess; // rust是静态语言，编译时必须知道所有数据的具体类型，然而此处编译器可以推断出guess的数据类型为i32，所以不用指定guess变量类型，但是把这行去掉，不能推断guess变量数据类型，那么会报错，这时就需要显式指定guess变量类型
    println!("{}", test_guess);
}
