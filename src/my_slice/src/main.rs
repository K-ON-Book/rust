fn main() {
    let mut s = String::from("Hello World");
    let word_index = first_world(&s);
    s.clear();
    println!("{}  {}", word_index, s);
    let array = [2; 7];
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
