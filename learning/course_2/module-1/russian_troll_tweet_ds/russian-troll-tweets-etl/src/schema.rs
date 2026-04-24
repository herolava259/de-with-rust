use core::time;

use chrono::{DateTime, Utc};

use std::hash::{Hash, Hasher};

use  std::collections::{HashMap};

#[derive(Debug)]
pub struct TweetAggregateRoot
{
    pub tweet_id: String,
    pub screen_name: String,
    pub tweet_text: String,
    pub user_id: String,
    pub permalink: String,
    pub timestamp: DateTime<Utc>,
    pub hashtags: Vec<(String, String)>,
    pub links: Vec<(String, String)>
}

pub struct TweetBuilder
{
    hashtags_table: HashMap<String, String>,
    links_table: HashMap<String, String>,
    tweet_id: Option<String>,
    screen_name: Option<String>,
    tweet_text: Option<String>,
    user_id: Option<String>,
    timestamp: Option<String>,
    permalink: Option<String>
}



pub struct TweetEntity
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

pub struct HashtagEntity
{
    tag: String,
    archieved_url: String
}

pub struct LinkEntity
{
    url: String,
    archived_url: String,
    tweet_id: String
}

pub struct UserEntity
{
    user_id: String,
    screen_name: String
}

impl Hash for HashtagEntity{

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl Hash for UserEntity {
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

    pub fn normalize_schema(self) -> (TweetEntity, Vec<TweetHashTagRelationship>,Vec<HashtagEntity>, Vec<LinkEntity>, UserEntity )
    {
        let tweet = TweetEntity {
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
            HashtagEntity{
                tag: tag,
                archieved_url: url
            }
        }).collect();

        let links = self.links.into_iter().map(|url, archived_url| {
            LinkEntity {
                url: url,
                archived_url: archived_url,
                tweet_id: self.tweet_id
            }
        }).collect();

        let user = UserEntity{
            user_id: self.user_id,
            screen_name: self.screen_name
        };

        (tweet, hashtag_rels, hashtags, links, users)
    }
}

pub impl TweetBuilder {

    pub fn new() -> Self {
        Self {
            tweet_id: None,
            screen_name: None,
            tweet_text: None,
            user_id: None,
            timestamp: None,
            permalink: None,
            hashtags_table: HashMap::new(),
            links_table: HashMap::new()
        }
    }

    pub fn with_tweet_id(mut self, id: String) -> Self
    {
        self.tweet_id = Some(id);
        self

    }

    pub fn with_screen_name(mut self, name: String) -> Self
    {
        self.screen_name = Some(name);
        self
    }

    pub fn with_tweet_content(mut self, content: String) -> Self 
    {
        self.tweet_text = Some(content);
        self
    }

    pub fn with_user_id(mut self, id: String) -> Self
    {
        self.user_id = Some(id);
        self

    }

    pub fn with_timestamp(mut self, ts: String) -> Self
    {
        self.timestamp = Some(ts);
        self
    }

    pub fn with_permalink(mut self, permalink: String) -> Self
    {
        self.permalink = Some(permalink);
        self
    }

    pub fn has_hashtag(mut self, tag: String, archieved_url: String) -> Self
    {
        self.hashtags_table[tag] = archieved_url;
        self
    }

    pub fn has_link(mut self, link: String, archieved_url: String) -> Self
    {
        self.hashtags_table[link] = archieved_url;
        self
    }

    pub fn build(self) -> TweetAggregateRoot
    {
        TweetAggregateRoot::new(self.tweet_id.unwrap_or(String::new()), 
                             self.screen_name.unwrap_or(String::new()), 
                             self.tweet_text.unwrap_or(String::new()), 
                              self.user_id.unwrap_or(String::new()), 
                              self.timestamp.unwrap_or(String::new()), 
                             self.hashtags_table.into_iter().map(|(k, v)| (k, v)).collect(),  
                             self.links_table.into_iter().map(|(k, v)| (k, v)).collect(), 
                             self.permalink.unwrap_or(String::new()) )
    }
}

