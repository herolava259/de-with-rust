use tokio::sync::mpsc;
use crate::load::Neo4jConfiguration;
use crate::{extract, transform, load};
use crate::schema::TweetAggregateRoot;
use crate::error::EtlError;

const CHANNEL_BUFFER: usize = 100;

fn build_pipeline_channels() -> (mpsc::Sender<String>,
                                mpsc::Receiver<String>,
                                mpsc::Sender<Vec<TweetAggregateRoot>>,
                                mpsc::Receiver<Vec<TweetAggregateRoot>>)
{
    let (tx_extract,   rx_extract)   = mpsc::channel::<String>(CHANNEL_BUFFER);
    let (tx_transform, rx_transform) = mpsc::channel::<Vec<TweetAggregateRoot>>(CHANNEL_BUFFER);
    (tx_extract, rx_extract, tx_transform, rx_transform)
}

// Each ? auto-converts stage errors → EtlError via #[from]
pub async fn etl_pipeline_to_csv_to_import_neo4j(export_dir: String) -> Result<[String; 5], EtlError>
{
    let (tx_extract, rx_extract, tx_transform, rx_transform) = build_pipeline_channels();

    let (extract_res, transform_res, load_res) = tokio::join!(
        extract::extract_html(tx_extract),
        transform::transform_to_tweet_aggregate(rx_extract, tx_transform),
        load::export_csv_to_import(export_dir, rx_transform),
    );

    extract_res.map_err(EtlError::Extract)?;
    transform_res.map_err(EtlError::Transform)?;
    let paths = load_res.map_err(EtlError::Load)?;

    Ok(paths)
}

pub async fn etl_pipeline_to_csv_to_archieve(path_dir: String, delimiter: u8, ) -> Result<String, EtlError>
{
    let (tx_extract, rx_extract, tx_transform, rx_transform) = build_pipeline_channels();

    let (extract_res, transform_res, load_res) = tokio::join!(
        extract::extract_html(tx_extract),
        transform::transform_to_tweet_aggregate(rx_extract, tx_transform),
        load::export_denormalized_data_to_csv(path_dir, delimiter, rx_transform),
    );

    extract_res.map_err(EtlError::Extract)?;
    transform_res.map_err(EtlError::Transform)?;
    let path = load_res.map_err(EtlError::Load)?;

    Ok(path)
}

pub async fn etl_pipeline_to_neo4j_end2end(config:Neo4jConfiguration) -> Result<(), EtlError>
{
    let (tx_extract, rx_extract, tx_transform, rx_transform) = build_pipeline_channels();

    let (extract_res, transform_res, load_res) = tokio::join!(
        extract::extract_html(tx_extract),
        transform::transform_to_tweet_aggregate(rx_extract, tx_transform),
        load::load_to_neo4j(config, rx_transform),
    );

    extract_res.map_err(EtlError::Extract)?;
    transform_res.map_err(EtlError::Transform)?;
    load_res.map_err(EtlError::Load)?;

    Ok(())
}