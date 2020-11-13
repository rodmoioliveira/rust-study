use std::env;

fn main() {
    let args: Vec<f64> = env::args()
        .filter_map(|s| s.parse::<f64>().ok())
        .filter(|i| i != &0.0)
        .collect();

    match args.len() {
        0 => println!("{}", 0),
        1 => println!("{}", args[0]),
        _ => {
            let mut args_iter = args.iter();
            let first = args_iter.next().unwrap().clone();
            println!("{}", args_iter.fold(first, |acc, cur| acc / cur));
        }
    };
}
