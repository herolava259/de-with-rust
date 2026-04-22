use std::collections::HashMap;

use reqwest::{self, Url, Client};

use std::time::Duration;

use serde_json::Result;


const LINKS_SOURCE: &str = "https://github.com/johnymontana/russian-twitter-trolls/blob/master/import/data/twitter_handle_urls.csv";
const VERIFY_LINKS_URL: &str = "http://archive.org/wayback/available";
const CONCURRENT_SIZE: usize = 5;

pub async fn load_archived_links() -> impl Iteratos<Item = String>
{
    

    let response = reqwest::get(LINKS_SOURCE)
                            .await?
                            .text()
                            .await?;

    response.split('\n').into_iter()
}

pub async fn verify_link_available(client: &Client, link: &String) -> bool
{
    let url = reqwest::Url::parse_with_params(Url, &[("url", link)])?;


    let response = client
                        .get(url)
                        .send()
                        .await?
                        .text()
                        .await?;
    
    serde_json::from_str::<serde_json::Value>(&response)
            .ok()
            .and_then(|json_obj| json_obj.get("archived_snapshots")
                                                .and_then(|v| v.get("closest")
                                                                        .cloned())
                                                                    )
            .is_some()
}


use async_trait::async_trait;
use futures::stream::{self, StreamExt, join_all};


#[async_trait]
pub trait FilterOutUnavailableLink: Sized {
    pub async fn filter_unavailable_links(self) -> Vec<String>;
}

#[async_trait]
impl<I> FilterOutUnavailableLink for I 
where I: IntoIterator<Item = String> + Send
{
    pub async fn filter_unavailable_links(self) -> Vec<String> {

        let client = Client::builder()
                                .connect_timeout(Duration::from_millis(100))
                                .build()?;
        

        stream::iter(self)
                // .chunks(CONCURRENT_SIZE)
                // .map(|chunk|{

                //     let client = & client;
                //     async move {
                //         let futures = chunk.iter()
                //                            .map(|link| verify_link_available(client, link));
                //         let availability = join_all(futures).await;
                        
                //         chunk.into_iter()
                //              .zip(availability.into_iter())
                //              .filter_map(|(link, avail)| if avail { Some(link) } else { None })
                //     }
                // })
                // .flatten()
                .map(|link| async {
                    if verify_link_available(&client, &link).await {
                        Some(link)
                    } else {
                        None
                    }
                })
                .buffer_unordered(CONCURRENT_SIZE)
                .filter_map(async move |x| x)
                .collect::<Vec<_>>()
                .await
    }
}