use tokio::sync::mpsc;
use csv::Writer;
use serde::Serialize;
use crate::schema::TweetAggregateRoot;
use std::{error::Error, fs::File, io::{BufReader, BufWriter}, iter::Rev, path::{Path, PathBuf}, time::Duration};
use chrono::{Utc, Local};



#[derive(Serialize)]
struct TweetNode
{
    tweet_id: String,
    text: String,
    permalink: String,
    author_id: String,
    timestamp: i64
}

#[derive(Serialize)]
pub struct TweetHashTagRelationship
{
    tweet_id: String,
    hashtag: String
}

#[derive(Serialize)]
pub struct UserTweetRelationship
{
    user_id: String,
    tweet_id: String
}


#[derive(Serialize)]
pub struct TweetLinkRelationship
{
    tweet_id: String,
    link: String
}


#[derive(Serialize)]
pub struct HashtagNode
{
    tag: String,
    archieved_url: String
}

#[derive(Serialize)]
pub struct LinkNode
{
    url: String,
    archived_url: String,
    tweet_id: String
}


#[derive(Serialize)]
pub struct UserNode
{
    user_id: String,
    screen_name: String
}


#[derive(Serialize)]
pub struct LinkColumn
{
    url: String,
    archived_url: String
}

impl TweetAggregateRoot
{
    fn decompose(self) -> (TweetNode, Vec<HashtagNode>, Vec<TweetHashTagRelationship>, UserTweetRelationship, Vec<TweetLinkRelationship>, Vec<LinkNode>, UserNode)
    {
        let tweet = TweetNode {
            tweet_id: self.tweet_id,
            text: self.tweet_text,
            permalink: self.permalink,
            author_id: self.user_id,
            timestamp: self.timestamp.timestamp()
        };

        let tweet_hashtag_rels = self.hashtags.into_iter().map(|(tag, l)| {
            TweetHashTagRelationship{
                tweet_id: self.tweet_id,
                hashtag: tag
            }
        }).collect();

        let user_tweet_rel = UserTweetRelationship{
            user_id: self.user_id,
            tweet_id: self.tweet_id

        };

        let tweet_link_rels = self.links.into_iter().map(|(url, archieved_url)|{
            TweetLinkRelationship{
                tweet_id: self.tweet_id,
                link: url,
            }
        }).collect();

        let hashtags = self.hashtags.into_iter().map(|(tag, url)| {
            HashtagNode{
                tag: tag,
                archieved_url: url
            }
        }).collect();

        let links = self.links.into_iter().map(|(url, archived_url)| {
            LinkNode {
                url: url,
                archived_url: archived_url,
                tweet_id: self.tweet_id
            }
        }).collect();

        let user = UserNode{
            user_id: self.user_id,
            screen_name: self.screen_name
        };

        (tweet, hashtags, tweet_hashtag_rels,user_tweet_rel, tweet_link_rels, links, user)

    }
}

pub fn export_denormalized_data_to_csv(path_dir: String,delimiter: u8, source: mpsc::Receiver<Vec<TweetAggregateRoot>>) -> Result<&str, Box<dyn std::error::Error>>
{
    let dest_file_name = format!("tweet_denomalized_data_{}.csv", Utc::now().format("%Y%m%dT%H%M%S").to_string());

    let mut file_path = Path::new(path_dir.as_str()).join(dest_file_name);

    let file = File::create(file_path)?;

    let csv_writer = csv::WriterBuilder::new()
                                    .delimiter(delimiter)
                                    .from_writer(BufWriter::new(file));

    csv_writer.write_record(&["tweet_id", "screen_name", "tweet_text", "user_id", "timestamp", "hashtags", "links", "permalink"]);

    while let Some(group) = source.blocking_recv(){

        for row in group.into_iter()
        {
            let hashtags_serialized = serde_json
                                    ::to_string_pretty(row.hashtags.into_iter().map(|(t, l)| {
                                        HashtagNode{
                                            tag: t,
                                            archieved_url: l
                                        }
                                    }).collect()).unwrap_or("[]".to_string());

            let links_serialized = serde_json::to_string_pretty(
                row.links.into_iter().map(|(l, al)| {
                    LinkColumn
                    {
                        url: l,
                        archived_url: al
                    }
                }).collect()
            ).unwrap_or("[]".to_string());

            csv_writer.write_record(&[row.tweet_id, 
                                      row.screen_name, 
                                      row.tweet_text, row.user_id, 
                                      row.timestamp.timestamp().to_string(), 
                                      hashtags_serialized, 
                                      links_serialized, 
                                      row.permalink])?;
        }
    }


    writer.flush()?;

    OK(file_path.to_str().unwrap().to_string())

}

use tokio::sync::mpsc;
use tokio::task::JoinSet;
use std::collections::HashSet;


trait CsvRecord {
    type Key: Eq + Hash;
    fn dedup_key(&self) -> Self::Key;
    fn to_record(&self) -> Vec<String>;
}

fn spawn_csv_writer<T>(
    set_join: &mut JoinSet<Result<(), ()>>,
    path: PathBuf,
    headers: &'static [&'static str],
    mut rtx: mpsc::Receiver<Vec<T>>,
)
where
    T: CsvRecord + Send + 'static
{
    set_join.spawn_blocking(move || {
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .from_writer(BufWriter::new(File::create(path).map_err(|_| ())?));

        writer.write_record(headers).map_err(|_| ())?;

        let mut seen = HashSet::new();
        while let Some(batch) = rx.blocking_recv() {
            for row in batch {
                if seen.insert(row.dedup_key()) {
                    writer.write_record(row.to_record()).map_err(|_| ())?;
                }
            }
        }

        writer.flush().map_err(|_| ())
    });
}


impl CsvRecord for TweetNode {
    type Key = String;
    fn dedup_key(&self) -> String { self.tweet_id }
    fn to_record(&self) -> Vec<String> {
        vec![self.tweet_id.to_string(), self.text.clone(),
             self.permalink.clone(), self.author_id.to_string(),
             self.timestamp.to_string()]
    }
}

impl CsvRecord for HashtagNode {
    type Key = String;
    fn dedup_key(&self) -> String { self.tag.clone() }
    fn to_record(&self) -> Vec<String> { vec![self.tag.clone(), self.archieved_url.clone()] }
}

impl CsvRecord for TweetHashTagRelationship {
    type Key = String;
    fn dedup_key(&self) -> String { format!("{}$-${}", self.tweet_id, self.hashtag) }
    fn to_record(&self) -> Vec<String> {
        vec![self.tweet_id, self.hashtag, "HAS_TAG".to_string()]
    }
}

impl CsvRecord for TweetLinkRelationship {
    type Key = String;
    fn dedup_key(&self) -> String { format!("{}$-${}", self.tweet_id, self.link) }
    fn to_record(&self) -> Vec<String> {
        vec![self.tweet_id, self.link ,"HAS_LINK".to_string()]
    }
}


impl CsvRecord for UserTweetRelationship {
    type Key = String;
    fn dedup_key(&self) -> String { format!("{}$-${}", self.user_id, self.tweet_id) }
    fn to_record(&self) -> Vec<String> {
        vec![self.user_id, self.tweet_id, "POSTED".into()]
    }
}

impl CsvRecord for LinkNode {
    type Key = String;
    fn dedup_key(&self) -> String { self.url.clone() }
    fn to_record(&self) -> Vec<String> {
        vec![self.url.clone(), self.archived_url.clone(), self.tweet_id.to_string()]
    }
}

impl CsvRecord for UserNode {
    type Key = String;
    fn dedup_key(&self) -> String { self.user_id }
    fn to_record(&self) -> Vec<String> { vec![self.user_id.to_string(), self.screen_name.clone()] }
}

pub async fn export_csv_to_import(path_dir: String, source: mpsc::Receiver<Vec<TweetAggregateRoot>>) 
    -> Result<[String; 5], Box<dyn std::error::Error>>
{
    let time_label = Utc::now().format("%Y%m%dT%H%M%S").to_string();

    let dest = Path::new(&path_dir);

    let paths = [
        dest.join(format!("tweet_import_nodes_{}.csv", time_label)),
        dest.join(format!("hashtag_import_nodes_{}.csv", time_label)),
        dest.join(format!("hashtag_tweet_import_relationships_{}.csv", time_label)),
        dest.join(format!("user_tweet_import_relationships_{}.csv", time_label)),
        dest.join(format!("tweet_link_import_relationships_{}.csv", time_label)),
        dest.join(format!("link_import_nodes_{}.csv", time_label)),
        dest.join(format!("user_import_nodes_{}.csv", time_label)),
    ];


    let (tweet_tx,   tweet_rx)   = mpsc::channel::<Vec<TweetNode>>(100);
    let (hashtag_tx, hashtag_rx) = mpsc::channel::<Vec<HashtagNode>>(100);
    let (tweet_hashtag_rel_tx,     tweet_hashtag_rel_rx)     = mpsc::channel::<Vec<TweetHashTagRelationship>>(100);
    let (user_tweet_rel_tx,     user_tweet_rel_rx)     = mpsc::channel::<Vec<UserTweetRelationship>>(100);
    let (tweet_link_rel_tx,     tweet_link_rel_rx)     = mpsc::channel::<Vec<TweetLinkRelationship>>(100);
    let (link_tx,    link_rx)    = mpsc::channel::<Vec<LinkNode>>(100);
    let (user_tx,    user_rx)    = mpsc::channel::<Vec<UserNode>>(100);

    while let Some(batch) = source.recv().await{
        let parts: Vec<_> = batch.into_iter().map(|v| v.decompose()).collect();

        tweet_tx  .send(parts.iter().map(|p| p.0.clone()).collect()).await?;
        hashtag_tx.send(parts.iter().flat_map(|p| p.1.clone()).collect()).await?;
        tweet_hashtag_rel_tx    .send(parts.iter().flat_map(|p| p.2.clone()).collect()).await?;
        user_tweet_rel_tx    .send(parts.iter().map(|p| p.3.clone()).collect()).await?;
        tweet_link_rel_tx    .send(parts.iter().flat_map(|p| p.4.clone()).collect()).await?;
        link_tx   .send(parts.iter().flat_map(|p| p.5.clone()).collect()).await?;
        user_tx   .send(parts.iter().flat_map(|p| p.6.clone()).collect()).await?;

    }

    drop((tweet_tx, hashtag_tx, tweet_hashtag_rel_tx, user_tweet_rel_tx, tweet_link_rel_tx , link_tx, user_tx));

    let mut set = JoinSet::new();


    spawn_csv_writer(&mut set, paths[0].clone(), 
        &["tweetId:ID(Tweet-ID){id-type:long}", "text:string", "permalink:string", "authorId:long", "timestamp:long"],
        tweet_rx);

    spawn_csv_writer(&mut set, paths[1].clone(),
        &["tag:ID(Hashtag-ID){id-type:string}", "archievedUrl:string"],
        hashtag_rx);

    spawn_csv_writer(&mut set, paths[2].clone(),
        &[":START_ID(Tweet-ID)", ":END_ID(Hashtag-ID)", ":TYPE"],
        tweet_hashtag_rel_rx);
    
    spawn_csv_writer(&mut set, paths[3].clone(),
        &[":START_ID(User-ID)", ":END_ID(TWEET-ID)", ":TYPE"],
        user_tweet_rel_rx);

    spawn_csv_writer(&mut set, paths[4].clone(),
        &[":START_ID(Tweet-ID)", ":END_ID(Link-ID)", ":TYPE"],
        tweet_link_rel_rx);

    spawn_csv_writer(&mut set, paths[5].clone(),
        &["url:ID(Link-ID){id-type:string}", "archievedUrl:string", "tweetId:ID(Tweet-URL-ID){id-type:long}"],
        link_rx);

    spawn_csv_writer(&mut set, paths[6].clone(),
        &["userId:ID(User-ID){id-type:long}", "screenName:string"],
        user_rx);

    while let Some(res) = set.join_next().await {
        res.unwrap().map_err(|_| "Error while exporting to CSV")?;
    }


    Ok(paths.map(|p| p.to_str().unwrap().to_string()))


}


use neo4j::{Neo4jError, ValueSend, address::Address, transaction::TransactionTimeout};
use neo4j::driver::auth::AuthToken;
use neo4j::driver::{ConnectionConfig, Driver, DriverConfig, RoutingControl};
use neo4j::retry::ExponentialBackoff;
use neo4j::{value_map, ValueReceive};
use neo4j::session::{Session, SessionConfig};
use std::sync::Arc;
use std::collections::HashMap;





pub struct Neo4jConfiguration
{
    pub database: &str,
    pub user: &str,
    pub host: &str,
    pub port: i16,
    pub password: &str,
    pub pool_size: usize,
    pub connection_timeout: Duration,
    pub transaction_timeout: TransactionTimeout
}

impl Neo4jConfiguration
{
    pub fn create_driver(self) -> Driver
    {
        //let database = Arc::new(String::from(self.database));
        let address = Address::from((self.host, self.port));

        let auth_token = AuthToken::new_basic_auth(self.user, self.password);

        Driver::new(
            ConnectionConfig::new(address),
            DriverConfig::new().with_auth(Arc::new(auth_token))
                                       .with_max_connection_pool_size(self.pool_size)
                                       .with_connection_timeout(self.connection_timeout),
        )
    }
}

pub struct Neo4jConnector
{
    configuration: Neo4jConfiguration,
    driver: Driver,
    retry_policy: ExponentialBackoff,
    database: Arc<String>
}

impl Neo4jConnector
{
    pub fn new(config: Neo4jConfiguration, retry_policy: Option<ExponentialBackoff>) -> Self
    {
        Self {
            configuration: config,
            retry_policy: retry_policy.unwrap_or(ExponentialBackoff::default()),
            driver: config.create_driver(),
            database: Arc::new(String::from(config.database))
        }
    }

    
    pub fn execute_query(self, query: &str,
                                parameters: HashMap<String, ValueSend>, 
                                routing_control: RoutingControl,
                                            ) -> Result<ValueReceive, &str>
    {

        
        let mut session = self.driver.session(SessionConfig::new().with_database(Arc::clone(&self.database)));

        let scalar = session.transaction()
               .with_routing_control(routing_control)
               .with_transaction_timeout(self.configuration.transaction_timeout)
               .run_with_retry(self.retry_policy.clone(), |tx|
            {
                Ok(tx.query(query)
                     .with_parameters(parameters)
                     .run()?
                     .try_as_eager_result()?
                     .expect("Some thing occur!")
                     .into_scalar()
                     .unwrap())
            });

        match scalar {

            Ok(rec) => Ok(rec),
            Err(_) => Err("Timeout or retry error!!")
            
        }

    }
}


pub async fn load_to_neo4j(config: Neo4jConfiguration, source: mpsc::Receiver<Vec<TweetAggregateRoot>>) -> Result<(), &str>
{

}