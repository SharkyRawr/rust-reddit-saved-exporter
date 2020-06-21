use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::prelude::*;

mod config;
mod my_oauth;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    match my_oauth::get_reddit_token(
        config::REDDIT_USERNAME,
        config::REDDIT_PASSWORD,
        config::REDDIT_APPID,
        config::REDDIT_SECRET,
    ).await {
        Ok(a) => {
            println!("Token: {}", a.access_token)
        },
        Err(err) => {
            println!("{:?}", err);
        }
    };

    /*let token = my_oauth::get_reddit_token(
        config::REDDIT_USERNAME,
        config::REDDIT_PASSWORD,
        config::REDDIT_APPID,
        config::REDDIT_SECRET,
    )
    .await
    .unwrap();*/

    //println!("Token: {}", token.access_token);
    
}
