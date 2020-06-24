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
    pub post_hint: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub selftext: Option<String>,
    #[serde(default)]
    pub selftext_html: Option<String>,

    // t1_ posts use body, body_html and link_url for some reason 🤷
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub body_html: Option<String>,
    #[serde(default)]
    pub link_url: Option<String>,
    #[serde(default)]
    pub link_title: Option<String>,
    #[serde(default)]
    pub link_id: Option<String>,
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

#[derive(Debug)]
pub enum RedditListingKind {
    Comment,    // t1
    Account,    // t2
    Link,       // t3
    Message,    // t4
    Subreddit,  // t5
    Award       // t6
}

impl RedditListingKind {
    pub fn from_str(s: &str) -> Option<RedditListingKind> {
        match s {
            "t1" => Some(RedditListingKind::Comment),
            "t2" => Some(RedditListingKind::Account),
            "t3" => Some(RedditListingKind::Link),
            "t4" => Some(RedditListingKind::Message),
            "t5" => Some(RedditListingKind::Subreddit),
            "t6" => Some(RedditListingKind::Award),
            _ => None
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RedditListingKind::Comment => "t1",
            RedditListingKind::Account => "t2",
            RedditListingKind::Link => "t3",
            RedditListingKind::Message => "t4",
            RedditListingKind::Subreddit => "t5",
            RedditListingKind::Award => "t6"
        }
    }
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

        let body = res.text().await.unwrap();
        //print!("{}", body);

        //let listing: RedditListing = res.json().await?;
        let listing: RedditListing = serde_json::from_str(&body).unwrap();
        //print!("{:?}", listing);

        Ok(listing)
    }
}
