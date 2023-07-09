use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let mut array: Vec<i32> = Vec::new();
    for _ in 0..5 {
        array.push(rand::thread_rng().gen_range(1..100));
    }
    buble_sort(&mut array);
    for index in 0..array.len() {
        print!("{:<3}", array[index]);
    }
}
fn buble_sort(array: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 0..array.len() {
        for j in (i + 1)..array.len() {
            // method 1:
            /* if array[i] > array[j] {
                let tmp = array[j];
                array[j] = array[i];
                array[i] = tmp;
            } */
            // method 2:
            match array[i].cmp(&array[j]) {
                Ordering::Greater => {
                    let tmp = array[j];
                    array[j] = array[i];
                    array[i] = tmp;
                }
                _ => {}
            }
        }
    }
    array
}
