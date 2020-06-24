use tokio::prelude::*;

mod config;
mod reddit;

#[tokio::main]
async fn main() {
    let r = reddit::Reddit::new(
        config::REDDIT_USERNAME,
        config::REDDIT_PASSWORD,
        config::REDDIT_APPID,
        config::REDDIT_SECRET,
    ).await.unwrap();

    let saved_posts: reddit::RedditListing;// = r.get_saved_posts().await.unwrap();

    match r.get_saved_posts().await {
        Ok(a) => saved_posts = a,
        Err(err) => {
            print!("Error fetching reddit saves!\n{:?}", err);
            return;
        }
    }

    for post in saved_posts.data.children {
        let post = post.data;
        println!("[{}] {}\n -> {}", post.name.unwrap_or_default(), post.title.unwrap_or_default(), post.url.unwrap_or_default());
    }
}
