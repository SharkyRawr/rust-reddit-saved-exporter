use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::prelude::*;

mod config;
mod reddit;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let r = reddit::Reddit::new(
        config::REDDIT_USERNAME,
        config::REDDIT_PASSWORD,
        config::REDDIT_APPID,
        config::REDDIT_SECRET,
    ).await.unwrap();

    let saved_posts = r.get_saved_posts().await.unwrap();

    for post in saved_posts.data.children {
        let post = post.data;
        println!("{} - {}", post.title, post.url);
    }
}
