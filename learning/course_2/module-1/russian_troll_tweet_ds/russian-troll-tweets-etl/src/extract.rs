use std::collections::HashMap;

use reqwest::{self, Url, Client};

use std::time::Duration;

use serde_json::Result;


const LINKS_SOURCE: &str = "https://github.com/johnymontana/russian-twitter-trolls/blob/master/import/data/twitter_handle_urls.csv";
const VERIFY_LINKS_URL: &str = "http://archive.org/wayback/available";
const CONCURRENT_SIZE: usize = 5;
const ARCHIVED_DOMAIN_URL: &str = "http://web.archive.org/";
const HTML_QUEUE_SIZE: usize = 100;

pub async fn load_archived_links() -> impl IntoIterator<Item = String>
{
    

    let response = reqwest::get(LINKS_SOURCE)
                            .await?
                            .text()
                            .await?;

    response.split('\n').into_iter()
}

pub async fn get_link_available(client: &Client, link: &String) -> Option<String>
{
    let url = reqwest::Url::parse_with_params(VERIFY_LINKS_URL, &[("url", link)])?;


    let response = client
                        .get(url)
                        .send()
                        .await?
                        .text()
                        .await?;
    
    let closest_obj = serde_json::from_str::<serde_json::Value>(&response)
            .ok()
            .and_then(|json_obj| json_obj.get("archived_snapshots")
                                                .and_then(|v| v.get("closest")
                                                                        .cloned())
                                                                    );
    if let Some(v) = closest_obj
    {
        return Some(v.to_string())
    }
    else {None } 
}



use futures::{Stream, stream::{self, StreamExt, join_all}};
use std::pin::Pin;

//type BoxStream<T> = Piusingn<Box<dyn Stream<Item = T> + Send>>;


pub trait TakeAvailableLink: Sized {
    async fn take_available_links(self) -> impl Stream<Item = String>;
}

impl<I> TakeAvailableLink for I 
where I: IntoIterator<Item = String> + Send + 'static
{
    async fn take_available_links(self) -> impl Stream<Item = String> {

        let client = Client::builder()
                                .connect_timeout(Duration::from_millis(100))
                                .build()
                                .unwrap();
        

        let stream = stream::iter(self.into_iter())
                .map(move |link| {
                    let client = client.clone();
                    async move {
                        get_link_available(&client, &link).await
                    }
                })
                .buffer_unordered(CONCURRENT_SIZE)
                .filter_map(|x| async move {x})
                ;
        
        stream
                
    }
}

pub async fn load_html(client: &Client, url: String) -> String
{
    client.get(url)
          .send()
          .await?
          .text()
          .await?
}


use tokio::sync::mpsc;


pub async fn extract_html(tx: mpsc::Sender<String>)
{
    let urls = load_archived_links().await;
    let mut stream = urls.take_available_links().await;

    let client = Client::builder()
                                .connect_timeout(Duration::from_millis(100))
                                .build()
                                .unwrap();

    while let Some(url) = stream.next().await{

        match tx.reserve().await {
            Ok(permit) => permit.send(load_html(&client, url).await),
            Err(_) => break
        }
    }

    // stream.for_each(|url| {
    //     let tx = tx.clone();

    //     async move {
    //         let _ = tx.send(item).await;
    //     }
    // }).await;
                                     
}

