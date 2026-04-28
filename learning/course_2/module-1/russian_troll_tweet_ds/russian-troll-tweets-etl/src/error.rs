use thiserror::Error;
use tokio;
use scraper;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("Failed to read file '{path}': {source}")]
    HttpRequest {
        path: String,
        #[source]
        source: Error,
    },

    #[error("Failed to parse HTML at '{path}': {reason}")]
    HtmlParse {
        path: String,
        reason: String,
    },

    #[error("Channel send failed — downstream closed")]
    ChannelClosed,
}



#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Missing required field '{field}' in tweet")]
    MissingField { field: &'static str },

    #[error("Failed to parse field '{field}': {reason}")]
    FieldParse {
        field: &'static str,
        reason: String,
    },

    #[error("Failed to parse to html-selector.")]
    ScrapeParseSelector,

    #[error("Channel receive failed — upstream closed unexpectedly")]
    UpstreamClosed,

    #[error("Channel send failed — downstream closed")]
    ChannelClosed,
}


#[derive(Debug, thiserror::Error)]
pub enum LoadToNeo4JError
{
    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Retry/timeout exhausted: {0}")]
    RetryExhausted(String),
 
    #[error("Scalar extraction failed")]
    ScalarExtraction,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Failed to create output directory '{path}': {source}")]
    CreateDir {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create CSV file '{path}': {source}")]
    FileCreate {
        path: String,
        #[source]
        source: std::io::Error,
    },


    #[error("Failed to flush data to csv file 'path': {source}")]
    FileFlush{
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write CSV record: {source}")]
    CsvWrite {
        #[source]
        source: csv::Error,
    },

    #[error("Failed to wait write file downstream task: {source}")]
    WaitingWriteFileTask
    {
        #[source]
        source: tokio::runtime::task::JoinError
    },

    #[error("Failed to create nodes/relationships: {source}")]
    Neo4jCreate {
        #[source]
        source: LoadToNeo4JError
    },

    #[error("Channel receive failed — upstream closed unexpectedly")]
    UpstreamClosed,


}


#[derive(Debug, Error)]
pub enum EtlError {
    #[error("Extract stage failed: {0}")]
    Extract(#[from] ExtractError), // <- no #[from]

    #[error("Transform stage failed: {0}")]
    Transform(#[from] TransformError), // <- no #[from]

    #[error("Load stage failed: {0}")]
    Load(#[from] LoadError), // <- no #[from]
}


impl From<ExtractError> for EtlError {
    fn from(e: ExtractError) -> Self {
        EtlError::Extract(e)
    }
}

impl From<TransformError> for EtlError {
    fn from(e: TransformError) -> Self {
        EtlError::Transform(e)
    }
}

impl From<LoadError> for EtlError {
    fn from(e: LoadError) -> Self {
        EtlError::Load(e)
    }
}