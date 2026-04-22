use crate::scrape::scrape_tweet_data_each_link;
use crate::schema::TweetAggregateRoot;
use tokio::sync::{Semaphore, Semaphore};
use std::sync::Arc;


const CONCCURENCY_CPU_BOUND_SIZE: usize = 5;

pub async fn transform_to_tweet_aggrgate(from_html_buffer: mpsc::Receiver<String>, to_storage_buffer: mpsc::Sender<Vec<TweetAggregateRoot>>)
{
    let semaphore = Arc::new(Semaphore::new(CONCCURENCY_CPU_BOUND_SIZE));

    while let Some(html) = from_html_buffer.recv().await {

        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let record;
        tokio::task::spawn_blocking(move || {
            record = scrape_tweet_data_each_link(response);
        }).await;

        drop(permit);
        to_storage_buffer.send(record).await.unwrap();


    }
}