use tokio::sync::mpsc;
use csv::Writer;
use serde::Serialize;
use crate::schema::TweetAggregateRoot;
use std::{error::Error, fs::File, io::{BufReader, BufWriter}, path::{Path, PathBuf}};
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
    fn decompose(self) -> (TweetNode, Vec<HashtagNode>, Vec<TweetHashTagRelationship>, Vec<LinkNode>, UserNode)
    {
        let tweet = TweetNode {
            tweet_id: self.tweet_id,
            text: self.tweet_text,
            permalink: self.permalink,
            author_id: self.user_id,
            timestamp: self.timestamp.timestamp()
        };

        let relationships = self.hashtags.into_iter().map(|(tag, l)| {
            TweetHashTagRelationship{
                tweet_id: self.tweet_id,
                hashtag: tag
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

        (tweet, hashtags, relationships, links, user)

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
        vec![self.tweet_id.to_string(), self.hashtag.clone(), "ATTACHED_TO".into()]
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
        dest.join(format!("link_import_nodes_{}.csv", time_label)),
        dest.join(format!("user_import_nodes_{}.csv", time_label)),
    ];


    let (tweet_tx,   tweet_rx)   = mpsc::channel::<Vec<TweetNode>>(100);
    let (hashtag_tx, hashtag_rx) = mpsc::channel::<Vec<HashtagNode>>(100);
    let (rel_tx,     rel_rx)     = mpsc::channel::<Vec<TweetHashTagRelationship>>(100);
    let (link_tx,    link_rx)    = mpsc::channel::<Vec<LinkNode>>(100);
    let (user_tx,    user_rx)    = mpsc::channel::<Vec<UserNode>>(100);

    while let Some(batch) = source.recv().await{
        let parts: Vec<_> = batch.into_iter().map(|v| v.decompose()).collect();

        tweet_tx  .send(parts.iter().map(|p| p.0.clone()).collect()).await?;
        hashtag_tx.send(parts.iter().flat_map(|p| p.1.clone()).collect()).await?;
        rel_tx    .send(parts.iter().flat_map(|p| p.2.clone()).collect()).await?;
        link_tx   .send(parts.iter().flat_map(|p| p.3.clone()).collect()).await?;
        user_tx   .send(parts.iter().flat_map(|p| p.4.clone()).collect()).await?;

    }

    drop((tweet_tx, hashtag_tx, rel_tx, link_tx, user_tx));

    let mut set = JoinSet::new();



    while let Some(res) = set.join_next().await {
        
        match res.unwrap() {
            Ok(_) => continue,
            Err(_) => return Err("Some error while exporting to csv file.")
        }
    }

    spawn_csv_writer(&mut set, paths[0].clone(), 
        &["tweetId:ID(Tweet-ID){id-type:long}", "text:string", "permalink:string", "authorId:long", "timestamp:long"],
        tweet_rx);

    spawn_csv_writer(&mut set, paths[1].clone(),
        &["tag:ID(Hashtag-ID){id-type:string}", "archievedUrl:string"],
        hashtag_rx);

    spawn_csv_writer(&mut set, paths[2].clone(),
        &[":START_ID(Tweet-ID)", ":END_ID(Hashtag-ID)", ":TYPE"],
        rel_rx);

    spawn_csv_writer(&mut set, paths[3].clone(),
        &["url:ID(Link-ID){id-type:string}", "archievedUrl:string", "tweetId:ID(Tweet-URL-ID){id-type:long}"],
        link_rx);

    spawn_csv_writer(&mut set, paths[4].clone(),
        &["userId:ID(User-ID){id-type:long}", "screenName:string"],
        user_rx);

    while let Some(res) = set.join_next().await {
        res.unwrap().map_err(|_| "Error while exporting to CSV")?;
    }


    Ok(paths.map(|p| p.to_str().unwrap().to_string()))


}