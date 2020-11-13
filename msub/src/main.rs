use std::env;

fn main() {
    let sub: i128 = env::args()
        .filter_map(|s| s.parse::<i128>().ok())
        .fold(0, |acc, cur| acc - cur);
    println!("{}", sub);
}
