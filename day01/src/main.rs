// write a program to distinguish odd and even number from 1 to 20
fn main() {
    for i in 1..21 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} is {}", i, even_odd);
    }
}
