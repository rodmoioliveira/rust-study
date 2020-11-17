// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
// https://blog.rust-lang.org/2019/11/07/Async-await-stable.html
use futures::join;
use reqwest;
use serde::Deserialize;
use std::error::Error;
use std::{thread, time};

#[derive(Deserialize, Debug)]
struct Ip {
    origin: String,
}

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

async fn req() -> Result<Ip, Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<Ip>()
        .await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let r = first().await;
    println!("{}", r);
    let (s, t) = join!(second(), third());
    println!("{} {}", s, t);
    println!("{}", fourth().await);
    match req().await {
        Ok(v) => println!("{:?}", v),
        Err(e) => eprintln!("Application error: {}", e),
    }

    Ok(())
}
