extern crate reqwest;

use anyhow::Result;
use std::collections::HashMap;

use serde::Deserialize;

// This `derive` requires the `serde` dependency.
#[derive(Deserialize)]
pub struct RedditAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
}

pub async fn get_reddit_token(
    username: &str,
    password: &str,
    appid: &str,
    secret: &str,
) -> Result<RedditAuthResponse> {
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("username", username);
    map.insert("password", password);

    let client = reqwest::Client::new();
    let res = client
        .post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(appid, Some(secret))
        .send()
        .await?;

    println!("{:?}", res.text().await);
    Err(anyhow::anyhow!("derp"))

    //let auth_result = res.json::<RedditAuthResponse>().await?;

    //Ok(auth_result)
}
