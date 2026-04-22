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
    hashtags: Vec<String>,
    links: Vec<String>,
    permalink: String
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

pub struct HashtagAggregateRoot
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

impl Hash for HashtagAggregateRoot{

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
            timestamp: String, hashtags: Vec<String>, links: Vec<String>, permalink: String) -> Self
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

    pub fn normalize_schema(self) -> (TweetRecord, Vec<TweetHashTagRelationship>, Vec<LinkRecord>, UserRecord )
    {
        let tweet = TweetRecord {
            tweet_id: self.tweet_id,
            text: self.tweet_text,
            permalink: self.permalink,
            author_id: self.user_id,
            timestamp: self.timestamp
        };

        let hashtag_rels = self.hashtags.into_iter().map(|ht| {
            TweetHashTagRelationship{
                tweet_id: self.tweet_id,
                hashtag: ht
            }
        }).collect();

        let links = self.links.into_iter().map(|l| {
            LinkRecord {
                url: l,
                archived_url: self.permalink,
                tweet_id: self.tweet_id
            }
        }).collect();

        let user = UserRecord{
            user_id: self.user_id,
            screen_name: self.screen_name
        };

        (tweet, hashtag_rels, links, user)
    }
}

