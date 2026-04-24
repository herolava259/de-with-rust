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
    headers: &'static [&'static str]
)
{
    
}

pub async fn export_csv_to_import(path_dir: String, source: mpsc::Receiver<Vec<TweetAggregateRoot>>) 
    -> Result<(&str, &str, &str, &str, &str), Box<dyn std::error::Error>>
{
    let time_label = Utc::now().format("%Y%m%dT%H%M%S").to_string();

    let dest_dir = Path::new(path_dir.as_str());

    let tweet_export_path = dest_dir.join(format!("tweet_import_nodes_{}.csv", time_label));
    let hashtag_export_path = dest_dir.join(format!("hashtag_import_nodes_{}.csv", time_label));
    let hashtag_tweet_rels_path = dest_dir.join(format!("hashtag_tweet_import_relationships_{}.csv", time_label));
    let link_export_path = dest_dir.join(format!("link_import_nodes_{}.csv", time_label));
    let user_export_path = dest_dir.join(format!("user_import_nodes_{}.csv", time_label));


    let (tweet_proc, tweet_cons) = mpsc::channel::<Vec<TweetNode>>(100);
    let (hashtag_proc, hashtag_cons) = mpsc::channel::<Vec<HashtagNode>>(100);
    let (ht_rel_proc, ht_rel_cons) = mpsc::channel::<Vec<TweetHashTagRelationship>>(100);
    let (link_proc, link_cons) = mpsc::channel::<Vec<LinkNode>>(100);
    let (user_proc, user_cons) = mpsc::channel::<Vec<UserNode>>(100);

    while let Some(batch) = source.recv().await{
        let decomposition = batch.into_iter().map(|v| v.decompose());

        let tweets = decomposition.map(|t| t.0).collect();

        tweet_proc.send(tweets).await.unwrap();

        let hashtags = decomposition.flat_map(|t| t.1).collect();

        hashtag_proc.send(hashtags).await.unwrap();

        let rels = decomposition.flat_map(|t| t.2).collect();

        ht_rel_proc.send(rels).await.unwrap();

        let links = decomposition.flat_map(|t| t.3).collect();

        link_proc.send(links).await.unwrap();

        let users = decomposition.flat_map(|t| t.4).collect();

        user_proc.send(users).await.unwrap();

    }

    drop(tweet_proc); drop(hashtag_proc); drop(ht_rel_proc); drop(link_proc); drop(user_proc);

    let mut set = JoinSet::new();

    set.spawn_blocking(move || {
        let file = File::create(tweet_export_path)?;

        let csv_writer = csv::WriterBuilder::new()
                                    .delimiter(b',')
                                    .from_writer(BufWriter::new(file));

        csv_writer.write_record(&["tweetId:ID(Tweet-ID){id-type:long}", "text:string", "permalink:string", "authoId:long", "timestamp:long"]);

        let mut existed = HashSet::new();

        while let Some(batch) = tweet_cons.blocking_recv()
        {
            for row in batch 
            {

                if ! existed.insert(row.tweet_id)
                {
                    continue;
                }
                
                match csv_writer.write_record(&[row.tweet_id, 
                                            row.text, 
                                            row.permalink, 
                                            row.author_id, 
                                            row.timestamp.to_string()]){

                    OK(_) => continue,
                    Err(_) => return Err(())
                };

                
            }
        }

        match csv_writer.flush()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }


    });


    set.spawn_blocking(move || {
        let file = File::create(hashtag_export_path)?;

        let csv_writer = csv::WriterBuilder::new()
                                    .delimiter(b',')
                                    .from_writer(BufWriter::new(file));

        csv_writer.write_record(&["tag:ID(Hashtag-ID){id-type:string}","archievedUrl:string"]);

        let mut existed = HashSet::new();

        while let Some(batch) = hashtag_cons.blocking_recv()
        {
            for row in batch 
            {

                if ! existed.insert(row.tag)
                {
                    continue;
                }

                match csv_writer.write_record(&[row.tag, row.archieved_url]){

                    OK(_) => continue,
                    Err(_) => return Err(())
                };

                //csv_writer.write_record(&[row.tag, row.archieved_url]);
            }
        }

        match csv_writer.flush()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }

    });

    set.spawn_blocking(move || {
        let file = File::create(hashtag_tweet_rels_path)?;

        let csv_writer = csv::WriterBuilder::new()
                                    .delimiter(b',')
                                    .from_writer(BufWriter::new(file));

        csv_writer.write_record(&[":START_ID(Tweet-ID)",":END_ID(Hashtag-ID)", ":TYPE"]);

        let mut existed = HashSet::new();

        while let Some(batch) = ht_rel_cons.blocking_recv()
        {
            for row in batch 
            {
                if ! existed.insert(format!("{}$-${}", row.tweet_id, row.hashtag))
                {
                    continue
                }
                match csv_writer.write_record(&[row.tweet_id, row.hashtag, String::from("ATTACHED_TO")]){
                    Ok(_) => continue,
                    Err(_) => Err(())
                };
            }
        }

        match csv_writer.flush()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }

    });


    set.spawn_blocking(move || {
        let file = File::create(link_export_path)?;

        let csv_writer = csv::WriterBuilder::new()
                                    .delimiter(b',')
                                    .from_writer(BufWriter::new(file));

        csv_writer.write_record(&["url:ID(Link-ID){id-type:string}","archievedUrl:string", "tweetId:ID(Tweet-URL-ID){id-type:long}"]);

        let mut existed = HashSet::new();

        while let Some(batch) = link_cons.blocking_recv()
        {
            for row in batch 
            {
                if ! existed.insert(row.url)
                {
                    continue;
                }
                match csv_writer.write_record(&[row.url, row.archived_url, row.tweet_id]){
                    Ok(_) => continue,
                    Err(_) => return Err(())
                };
            }
        }

        match csv_writer.flush()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }

    });

    set.spawn_blocking(move || {
        let file = File::create(user_export_path)?;

        let csv_writer = csv::WriterBuilder::new()
                                    .delimiter(b',')
                                    .from_writer(BufWriter::new(file));

        csv_writer.write_record(&["userId:ID(User-ID){id-type:long}","screenName:string"]);

        let mut existed = HashSet::new();

        while let Some(batch) = user_cons.blocking_recv()
        {
            for row in batch 
            {
                if ! existed.insert(row.user_id)
                {
                    continue
                }

                match csv_writer.write_record(&[row.user_id, row.screen_name]) {

                    OK(_) => continue,
                    Err(_) => return Err(())
                };

                //csv_writer.write_record(&[row.user_id, row.screen_name]);
            }
        }

        match csv_writer.flush()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }

    });

    while let Some(res) = set.join_next().await {
        
        match res.unwrap() {
            Ok(_) => continue,
            Err(_) => return Err("Some error while exporting to csv file.")
        }
    }


    Ok((tweet_export_path.to_str().unwrap(), 
        hashtag_export_path.to_str().unwrap(), 
        hashtag_tweet_rels_path.to_str().unwrap(),
        link_export_path.to_str().unwrap(),
        user_export_path.to_str().unwrap()))


}