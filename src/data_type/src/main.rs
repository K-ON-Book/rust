fn main() {
    // 标量类型: 整数类型、浮点类型，布尔类型，字符类型
    let guess: u32 = "9".parse().expect("Not a number");
    println!("{}", guess);

    let a = 11_000u32;
    println!("{}", a);
    let h = 0xffi32;
    println!("{}", h);
    let o = 0o77;
    println!("{}", o);
    let b = 0b0001_0000;
    println!("{}", b);

    let x = 2.0; //f64
    println!("{}", x);
    let y: f32 = 3.0;
    println!("{}", y);

    let ch = '😍'; // 4 bit
    println!("{}", ch);

    //
    let (m, n): (i32, u32) = (1, 9);
    println!("m:{} n:{}", m, n);
    // 复合类型
    //元组Tuple，
    let tupl: (i32, f64, u8) = (1, 1.0, 1);
    println!("{} {} {}", tupl.0, tupl.1, tupl.2);
    let (x, y, z) = tupl;
    println!("{} {} {}", x, y, z);
    // 数组
    let a: [i32; 5];
    a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a: [u32; 5] = [2; 5]; // let a:[u32;5] = [2,2,2,2,2]
    println!("{}", a[0]);
}
