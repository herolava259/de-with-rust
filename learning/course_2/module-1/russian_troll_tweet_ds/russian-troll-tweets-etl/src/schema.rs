use core::time;

use chrono::{DateTime, Utc};

use std::hash::{Hash, Hasher};
pub struct TweetAggregateRoot
{
    tweet_id: String,
    screen_name: String,
    tweet_text: String,
    user_id: String,
    timestamp: DateTime<Utc>,
    hashtags: Vec<(String, String)>,
    links: Vec<(String, String)>,
    permalink: String
}

pub struct TweetBuilder
{
    tweet_id: Option<String>,
    screen_name: Option<String>,
    tweet_text: Option<String>,
    user_id: Option<String>,
    time_stamp: Option<String>,
    permalink: Option<String>,
    hashtags_table: HashMap<String, String>,
    links_table: HashMap<String, String>
}



pub struct TweetRecord
{
    tweet_id: String,
    text: String,
    permalink: String,
    author_id: String,
    timestamp: DateTime<Utc>
}

pub struct TweetHashTagRelationship
{
    tweet_id: String,
    hashtag: String
}

pub struct HashtagRecord
{
    tag: String,
    archieved_url: String
}

pub struct LinkRecord
{
    url: String,
    archived_url: String,
    tweet_id: String
}

pub struct UserRecord
{
    user_id: String,
    screen_name: String
}

impl Hash for HashtagRecord{

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl Hash for UserRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
    }
}


impl TweetAggregateRoot
{
    pub fn new(tweet_id: String, screen_name: String, content: String, user_id: String,
            timestamp: String, hashtags: Vec<(String, String)>, links: Vec<(String, String)>, permalink: String) -> Self
    {
        Self
        {
            tweet_id: tweet_id,
            timestamp: timestamp.parse().expect("The format is invalid"),
            screen_name: screen_name,
            tweet_text: content,
            user_id: user_id,
            hashtags: hashtag,
            links: links,
            permalink: permalink
        }
    }

    pub fn normalize_schema(self) -> (TweetRecord, Vec<TweetHashTagRelationship>, Vec<LinkRecord>, Vec<>, UserRecord )
    {
        let tweet = TweetRecord {
            tweet_id: self.tweet_id,
            text: self.tweet_text,
            permalink: self.permalink,
            author_id: self.user_id,
            timestamp: self.timestamp
        };

        let hashtag_rels = self.hashtags.into_iter().map(|(tag, _)| {
            TweetHashTagRelationship{
                tweet_id: self.tweet_id,
                hashtag: tag
            }
        }).collect();

        let hashtags = self.hashtags.into_iter().map(|tag, url| {
            HashtagRecord{
                tag: tag,
                archieved_url: url
            }
        });

        let links = self.links.into_iter().map(|url, archived_url| {
            LinkRecord {
                url: url,
                archived_url: archived_url,
                tweet_id: self.tweet_id
            }
        }).collect();

        let user = UserRecord{
            user_id: self.user_id,
            screen_name: self.screen_name
        };

        (tweet, hashtag_rels, hashtags, links, users)
    }
}

impl TweetBuilder {

    fn new() -> Self {
        Self {
            tweet_id: None,
            screen_name: None,
            tweet_text: None,
            user_id: None,
            time_stamp: None,
            permalink: None,
            hashtags_table: HashMap<String, String>,
            links_table: HashMap<String, String>
        }
    }
}

