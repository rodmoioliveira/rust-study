// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
use futures::join;
use std::error::Error;
use std::{thread, time};

async fn first() -> i32 {
    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
    100
}

async fn second() -> i32 {
    let ten_millis = time::Duration::from_millis(2000);
    thread::sleep(ten_millis);
    200
}

async fn third() -> i32 {
    let ten_millis = time::Duration::from_millis(3000);
    thread::sleep(ten_millis);
    300
}

async fn fourth() -> i32 {
    let ten_millis = time::Duration::from_millis(4000);
    thread::sleep(ten_millis);
    400
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let r = first().await;
    println!("{}", r);
    let (s, t) = join!(second(), third());
    println!("{} {}", s, t);
    println!("{}", fourth().await);
    Ok(())
}
