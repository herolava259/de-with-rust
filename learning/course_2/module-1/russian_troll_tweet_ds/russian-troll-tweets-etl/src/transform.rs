use crate::scrape::scrape_tweet_data;
use crate::schema::TweetAggregateRoot;
use std::sync::{Arc};
use tokio::sync::{Semaphore, mpsc};
use crate::error::TransformError;


const CONCURENCY_CPU_BOUND_SIZE: usize = 5;

pub async fn transform_to_tweet_aggregate(from_html_buffer: mpsc::Receiver<String>, to_storage_buffer: mpsc::Sender<Vec<TweetAggregateRoot>>)
    -> Result<(), TransformError>
{
    let semaphore = Arc::new(Semaphore::new(CONCURENCY_CPU_BOUND_SIZE));

    while let Some(html) = from_html_buffer.recv().await {

        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let sender = to_storage_buffer.clone();

        tokio::task::spawn_blocking(move || {
            sender.blocking_send(scrape_tweet_data(response)?).map_err(|_| TransformError::ChannelClosed)?;
            drop(permit);
        });
    }

    Ok(())
}