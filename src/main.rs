
use serde::{Serialize, Deserialize};

mod my_oauth;

fn main() {
    println!("Hello, world!");
    let token = my_oauth::get_reddit_token().unwrap();
    println!("Token: {}", token);
}
