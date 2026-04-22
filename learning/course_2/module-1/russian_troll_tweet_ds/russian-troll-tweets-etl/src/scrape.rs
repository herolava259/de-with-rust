use tokio::sync::mpsc;
use schema::TweetAggregateRoot;
use reqwest::Client;
use scraper::{Selector, Html};
use std::collections::{HashMap};

pub async fn scrape_tweet_data(client: &Client, link: String) -> Vec<TweetAggregateRoot>
{
    let response = client.get(url)
                        .send()
                        .await?
                        .text()
                        .await?;

    let document = Html::parse_document(response.as_str());

    let tweet_selector = Selector::parse(r#"li[data-item-type="tweet"]"#).unwrap();
    let tweet_ctn_selector = Selector::parse(r#"div.tweet"#).unwrap();
    let tweet_content_selector = Selector::parse(r#"p.tweet-text"#).unwrap();


    for tw in document.select(&tweet_selector)
    {
        let data_map: HashMap<String, String + Vec<HashMap<String, String>>> = HashMap::new();

        data_map["tweet_id"] = tw.attr("data-item-id").unwrap_or("").to_string();

        let tw_cnt_div = tw.select(&tweet_ctn_selector).next().unwrap();

        data_map["screen_name"] = tw_cnt_div.attr("data-screen-name").unwrap_or("").to_string();

        data_map["permalink"] = tw_cnt_div.attr("data-permalink-path").unwrap_or("").to_string();

        let tw_content_p = tw_cnt_div.select(&tweet_content_selector).next().unwrap();

        data_map["tweet_text"] = tw_content_p.inner_html();

        data_map["user_id"] = tw_cnt_div.attr("data-user-id").unwrap_or("").to_string();

        let mut hashtags: Vec<String>= Vec![];

        let mut links: Vec<String> = Vec![];

        

    }

} 