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

    for child in saved_posts.data.children {
        let post = child.data;

        let t = reddit::RedditListingKind::from_str(&child.kind).unwrap();

        match t {
            reddit::RedditListingKind::Comment => {
                let title = post.link_title.unwrap_or_default();
                println!("[{}] {}\n -> {}", post.name.unwrap_or_default(), title, post.link_url.unwrap_or_default());
                let outpath = format!("saved/{}", post.subreddit.replace("/", " or ").replace("\\", " or "));
                let outfilename = format!("{}/{}.md", outpath, title.replace("/", " or ").replace("\\", " or "));
                std::fs::create_dir_all(outpath).unwrap();
                std::fs::write(outfilename, post.body.unwrap()).unwrap();
            },
            reddit::RedditListingKind::Link => {
                let title = post.title.unwrap_or_default();
                println!("[{}] {}\n -> {}", post.name.unwrap_or_default(), title, post.url.unwrap_or_default());
                let outpath = format!("saved/{}", post.subreddit.replace("/", " or ").replace("\\", " or "));
                let outfilename = format!("{}/{}.md", outpath, title.replace("/", " or ").replace("\\", " or "));
                std::fs::create_dir_all(outpath).unwrap();
                std::fs::write(outfilename, post.selftext.unwrap()).unwrap();
            }
            _ => {
                println!("Unhandable post kind: {:?} {}", t, post.name.unwrap_or_default());
                break
            }
        }

        //
    }
}
