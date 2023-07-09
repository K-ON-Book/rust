use std::io;
fn main() {
    let mut sign_magnitude_1: Vec<i32> = Vec::new();
    // let sign_magnitude_1_copy;
    let mut sign_magnitude_2: Vec<i32> = Vec::new();
    // let sign_magnitude_2_copy;
    loop {
        let mut str = String::new();
        print!("Please input Sign-Magnitude_1(At least two bits): \n");
        io::stdin().read_line(&mut str).expect("Faild to read line");
        let str = str.trim();
        if is_od(&str) == false {
            continue;
        }
        if str.len() < 2 {
            continue;
        }
        let mut arr: Vec<char> = Vec::new();
        for (_, c) in str.chars().enumerate() {
            // arr[i] = c;
            arr.push(c);
        }
        sign_magnitude_1 = change(arr);
        // sign_magnitude_1_copy = sign_magnitude_1.clone();
        break;
    }
    loop {
        let mut str = String::new();
        print!("Please input Sign-Magnitude_2(At least two bits): \n");
        io::stdin().read_line(&mut str).expect("Faild to read line");
        let str = str.trim();
        if is_od(&str) == false {
            continue;
        }
        if str.len() < 2 {
            continue;
        }
        let mut arr: Vec<char> = Vec::new();
        for (_, c) in str.chars().enumerate() {
            // arr[i] = c;
            arr.push(c);
        }
        sign_magnitude_2 = change(arr);
        // sign_magnitude_2_copy = sign_magnitude_2.clone();
        break;
    }
    //计算两个原码相加结果（10进制）
    let s1_result_d = binnary_code_result(&mut sign_magnitude_1);
    let s2_result_d = binnary_code_result(&mut sign_magnitude_2);
    let sum_s1_s2_result_d =
        binnary_code_result(&mut sign_magnitude_2) + binnary_code_result(&mut sign_magnitude_1);
    println!(
        "(Decimal): {} + {} = {:+}",
        s1_result_d, s2_result_d, sum_s1_s2_result_d
    );
    //计算两个原码相加结果（2进制）
    println!();
    println!("<<<Calculate the sum of the two sign-magnitude>>>");
    let sum_s1_s2_result_b = add_both_bin(&mut sign_magnitude_1, &mut sign_magnitude_2);
    let mut sum_s1_s2_result_b_copy = sum_s1_s2_result_b.clone();
    println!(
        "(Binnary): {:?} + {:?} = {:?}\tresult to decimal: {:+}",
        sign_magnitude_1,
        sign_magnitude_2,
        sum_s1_s2_result_b,
        binnary_code_result(&mut sum_s1_s2_result_b_copy)
    );
    //原码转反码
    //计算两个反码相加
    println!();
    println!("<<<Calculate the sum of two inverse-code>>>");
    let sum_i1_i2_result_b = add_both_bin(
        &mut o_to_inverse_code(&sign_magnitude_1),
        &mut o_to_inverse_code(&sign_magnitude_2),
    );
    let mut sum_i1_i2_result_b_copy = sum_i1_i2_result_b.clone();
    println!(
        "(Binnary): {:?} + {:?} = {:?}\tresult to decimal: {:+}",
        o_to_inverse_code(&sign_magnitude_1),
        o_to_inverse_code(&sign_magnitude_2),
        sum_i1_i2_result_b,
        binnary_code_result(&mut sum_i1_i2_result_b_copy)
    );
    //原码转补码
    //计算两个补码相加
    println!();
    println!("<<<Calculate the sum of two complementary-code>>>");
    let sum_c1_c2_result_b = add_both_bin(
        &mut o_to_complementary_code(&sign_magnitude_1),
        &mut o_to_complementary_code(&sign_magnitude_2),
    );
    let mut sum_c1_c2_result_b_copy = sum_c1_c2_result_b.clone();
    println!(
        "(Binnary): {:?} + {:?} = {:?}\tresult to decimal: {}",
        o_to_complementary_code(&sign_magnitude_1),
        o_to_complementary_code(&sign_magnitude_2),
        sum_c1_c2_result_b,
        binnary_code_result(&mut sum_c1_c2_result_b_copy)
    );
    //反码转原码
    println!();
    println!(
        "i_to_o: {:?}\t{:?}",
        i_to_original_code(&o_to_inverse_code(&sign_magnitude_1)),
        i_to_original_code(&o_to_inverse_code(&sign_magnitude_2))
    );
    //补码转原码
    println!();
    let test = vec![1, 0, 0, 0];
    println!(
        "{:?}\n{:?}\t{:?}",
        c_to_original_code(&test),
        c_to_original_code(&o_to_complementary_code(&sign_magnitude_1)),
        c_to_original_code(&o_to_complementary_code(&sign_magnitude_2))
    );

    return;
}

/// 原码转反码
/// 原理：对于原码是正数，反码等于原码;
///       对于原码是负数，反码为：符号位不变，其余位均按位取反
/// ```rust
/// let original_code1:Vec<i32> = Vec![1,0,1,1];
/// println!("{:?}",o_to_inverse_code(original_code1));
///
/// let original_code2:Vec<i32> = Vec![0,0,1,1];
/// println!("{:?}",o_to_inverse_code(original_code2));
/// ```
/// output: [1,1,0,0]
///         [0,0,1,1]
fn o_to_inverse_code(original_code: &Vec<i32>) -> Vec<i32> {
    let mut inverse_code: Vec<i32> = Vec::new();
    match original_code[0] {
        0 => {
            for (_, element) in original_code.iter().enumerate() {
                inverse_code.push(*element);
            }
        }
        1 => {
            for (index, element) in original_code.iter().enumerate() {
                if index > 0 {
                    match element {
                        0 => {
                            inverse_code.push(1);
                        }
                        1 => {
                            inverse_code.push(0);
                        }
                        _ => {
                            panic!();
                        }
                    }
                } else {
                    inverse_code.push(*element);
                }
            }
        }
        _ => {
            panic!();
        }
    }
    return inverse_code;
}

///反码转原码
///
///
fn i_to_original_code(inverse_code: &Vec<i32>) -> Vec<i32> {
    let mut original_code: Vec<i32> = vec![];
    for (index, element) in inverse_code.iter().enumerate() {
        match inverse_code[0] {
            0 => {
                original_code.push(*element);
            }
            1 => {
                if index > 0 {
                    match element {
                        0 => {
                            original_code.push(1);
                        }
                        1 => {
                            original_code.push(0);
                        }
                        _ => {
                            panic!();
                        }
                    }
                } else {
                    original_code.push(*element)
                }
            }
            _ => {
                panic!();
            }
        }
    }
    return original_code;
}

/// 原码转补码
/// 原理：原码是正数，补码等于原码
///       原码是负数，补码为原码的反码+1
///       
/// ```rust
/// let original_code1:Vec<i32> = Vec![1,0,1,1];
/// println!("{:?}",o_to_complementary_code(original_code1));
///
/// let original_code2:Vec<i32> = Vec![0,0,1,1];
/// println!("{:?}",o_to_complementary_code(original_code2));
/// ```
/// output: [1,1,0,1]
///         [0,0,1,1]
fn o_to_complementary_code(original_code: &Vec<i32>) -> Vec<i32> {
    let mut inverse_code = o_to_inverse_code(original_code);
    let mut complementary_code: Vec<i32> = Vec::new();

    match inverse_code[0] {
        0 => {
            for (_, element) in inverse_code.iter().enumerate() {
                complementary_code.push(*element);
            }
        }
        1 => {
            let mut tmp: Vec<i32> = vec![0, 1];
            //转置tmp、inverse_code,方便push 0到高位
            let mut tmp = transfer(&mut tmp);
            let mut inverse_code = transfer(&mut inverse_code);
            //比较 tmp 和 inverse_code 的位的长度，如果不一样就添0到长度小的高位上
            if tmp.len() > inverse_code.len() {
                for _ in 1..=tmp.len() - inverse_code.len() {
                    inverse_code.push(0);
                }
            } else {
                for _ in 1..=inverse_code.len() - tmp.len() {
                    tmp.push(0);
                }
            }
            //转置tmp、inverse_code
            let mut tmp = transfer(&mut tmp);
            let mut inverse_code = transfer(&mut inverse_code);
            complementary_code = add_both_bin(&mut inverse_code, &mut tmp);
        }
        _ => {
            panic!();
        }
    }
    return complementary_code;
}

/// 补码转原码
/// 原理：对补码再求一次补码
///       特殊：对于4bit, -8的补码是1000, 但是它没有原码和反码表示
fn c_to_original_code(complementary_code: &Vec<i32>) -> Vec<i32> {
    let mut original_code: Vec<i32> = Vec::new();
    match complementary_code[0] {
        1 => {
            for (index, element) in complementary_code.iter().enumerate() {
                if index > 0 {
                    match element {
                        0 => {
                            if index == complementary_code.len() - 1 {
                                let value = -1 * 2_i32.pow((complementary_code.len() - 1) as u32);
                                print!("this complementary-code can't show original-code, express by {}'s complementary-code<<=",value);
                                original_code = complementary_code.to_vec();
                                return original_code;
                            }
                        }
                        _ => {
                            original_code = o_to_complementary_code(&complementary_code);
                            return original_code;
                        }
                    }
                }
            }
        }
        _ => {
            original_code = o_to_complementary_code(&complementary_code);
            return original_code;
        }
    }
    return original_code;
}

fn transfer(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = arr.clone();
    let mut head = 0;
    let mut tail = arr.len() - 1;
    while head < tail {
        let temp = arr[head];
        arr[head] = arr[tail];
        arr[tail] = temp;
        head += 1;
        tail -= 1;
    }
    return arr.to_vec();
}
/// 数组类型 Vec<char> -> Vec<i32>
/// #example
/// ```rust
/// let mut arr:Vec<char>
/// println("{:?}",change(&arr))
/// ```
fn change(arr: Vec<char>) -> Vec<i32> {
    let numbers: Vec<i32> = arr.iter().map(|x| x.to_string().parse().unwrap()).collect();
    return numbers;
}

fn is_od(s: &str) -> bool {
    for element in s.chars() {
        match element {
            '0' | '1' => {}
            _ => {
                return false;
            }
        }
    }
    return true;
}

fn binnary_code_result(arr: &mut Vec<i32>) -> i32 {
    let arr = transfer(arr);
    let mut result: i32 = 0;
    for (index, element) in arr.iter().enumerate() {
        if arr.len() - 1 > index {
            let index: u32 = index as u32;
            result += element * (2_i32.pow(index));
        } else {
            if p_or_n(element) {
                result = result;
            } else {
                result = -result;
            }
        }
    }
    return result;
}

fn p_or_n(highest_position: &i32) -> bool {
    match highest_position {
        0 => {
            // println!("highest position is positive");
            return true;
        }
        1 => {
            // println!("highest position is negative");
            return false;
        }
        _ => {
            panic!()
        }
    }
}

/// 模拟二进制数相加
/// 思路：
fn add_both_bin(source1: &mut Vec<i32>, source2: &mut Vec<i32>) -> Vec<i32> {
    //转置两个source数组
    let mut source1 = transfer(source1);
    let mut source2 = transfer(source2);
    //比较两个source位的长度，如果不一样就添0到长度小的高位上
    if source1.len() > source2.len() {
        for _ in 1..=source1.len() - source2.len() {
            source2.push(0);
        }
    } else {
        for _ in 1..=source2.len() - source1.len() {
            source1.push(0);
        }
    }
    let mut carry: i32 = 0;
    let mut output: Vec<i32> = Vec::new();
    //定义标志变量,为了交换下面第一次循环中执行的语句顺序
    let mut need_to_swap = false;
    let mut index = 0;
    let mut cout: i32 = 0;
    let mut cin: i32 = 0;
    for (source1_element, source2_element) in source1.iter().zip(source2.iter()) {
        let bit_sum = source1_element + source2_element + carry;
        if need_to_swap {
            carry = bit_sum / 2;
            output.push(bit_sum % 2);
            if source1.len() - 1 == index {
                cout = carry;
            }
            if source1.len() - 2 == index {
                cin = carry;
            }
        } else {
            output.push(bit_sum % 2);
            carry = bit_sum / 2;
        }
        if need_to_swap == false {
            need_to_swap = !need_to_swap;
        }
        index = index + 1;
    }
    if cout ^ cin == 1 {
        println!("结果：发生溢出");
    }
    return transfer(&mut output);
}
//TODO: 设计一个简单的有符号二进制加法器(gui)
