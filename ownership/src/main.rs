// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn main() {
    let s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[6..11];
    println!("s1: {}, s2: {}", s1, s2);
}
