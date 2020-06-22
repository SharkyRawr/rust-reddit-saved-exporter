use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use reqwest::header::HeaderValue;

#[derive(Deserialize, Debug)]
pub struct RedditAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
}


#[derive(Deserialize, Debug)]
pub struct RedditPostData {
    pub subreddit: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct RedditListingChild {
    pub kind: String,
    pub data: RedditPostData
}

#[derive(Deserialize, Debug)]
pub struct RedditListingData {
    pub dist: i64,
    pub children: Vec<RedditListingChild>
}

#[derive(Deserialize, Debug)]
pub struct RedditListing {
    pub kind: String,
    pub data: RedditListingData
}

pub struct Reddit {
    pub authresponse: RedditAuthResponse,
    pub username: String
}

impl Reddit {
    pub async fn new(username: &str, password: &str, appid: &str, secret: &str) -> Result<Reddit> {
        let mut map = HashMap::new();
        map.insert("grant_type", "password");
        map.insert("username", username);
        map.insert("password", password);

        let client = reqwest::Client::new();
        let res = client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(appid, Some(secret))
            .form(&map)
            .send()
            .await?;

        let body = res.text().await?;
        //println!("{:?}", body);

        let auth_result: RedditAuthResponse = serde_json::from_str(body.as_str())?;

        Ok(Reddit {
            authresponse: auth_result,
            username: String::from(username)
        })
    }

    pub async fn get_saved_posts(self: &Self) -> Result<RedditListing> {
        let url = format!("https://oauth.reddit.com/user/{}/saved?limit=100", self.username);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("bearer {}", &self.authresponse.access_token))?);
        headers.insert("User-Agent", HeaderValue::from_str("RustRedditSavedExporter/0.1 by /u/ImASharkRawwwr")?);

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        /*let body = res.text().await.unwrap();
        print!("{}", body);*/

        let listing: RedditListing = res.json().await?;
        //print!("{:?}", listing);

        Ok(listing)
    }
}

pub fn get_saved_posts(username: &str) -> Result<Vec<String>> {
    Err(anyhow::anyhow!("derp"))
}
