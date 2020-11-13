use std::env;

fn main() {
    let sum: i128 = env::args().filter_map(|s| s.parse::<i128>().ok()).sum();
    println!("{}", sum);
}
