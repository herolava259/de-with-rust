use tokio::sync::mpsc;
use crate::schema::TweetAggregateRoot;
use reqwest::{Client, Url};
use scraper::{Selector, Html};
use std::collections::{HashMap};
use crate::schema::TweetBuilder;

const SCRAPE_URL: &str = "http://web.archive.org/web/20150603004258/";

pub fn scrape_tweet_data_each_link(response: String) -> Vec<TweetAggregateRoot>
{
    // let response = client.get(SCRAPE_URL + link)
    //                     .send()
    //                     .await?
    //                     .text()
    //                     .await?;

    let document = Html::parse_document(response.as_str());

    let tweet_selector = Selector::parse(r#"li[data-item-type="tweet"]"#).unwrap();
    let tweet_ctn_selector = Selector::parse(r#"div.tweet"#).unwrap();
    let tweet_content_selector = Selector::parse(r#"p.tweet-text"#).unwrap();
    let link_selector = Selector::parse(r#"a.twitter-timeline-link"#).unwrap();
    let hashtag_selector = Selector::parse(r#"a.twitter-hashtag"#).unwrap();
    let bold_tag_selector = Selector::parse(r#"b"#).unwrap();

    let mut result: Vec<TweetAggregateRoot> = Vec::new();


    for tw in document.select(&tweet_selector)
    {
        let builder = TweetBuilder::new();

        let tw_ctn_elem_ref = tw.select(&tweet_ctn_selector).next().unwrap();
        let tw_text_elem_ref = tw_ctn_elem_ref.select(&tweet_content_selector).next().unwrap();

        builder.with_tweet_id(tw.attr("data-item-id").unwrap_or("").to_string())
               .with_screen_name(tw_ctn_elem_ref.attr("data-screen-name").unwrap_or("").to_string())
               .with_permalink(tw_ctn_elem_ref.attr("data-permalink-path").unwrap_or("").to_string())
               .with_tweet_content(tw_text_elem_ref.inner_html())
               .with_user_id(tw_ctn_elem_ref.attr("data-user-id").unwrap_or("").to_string());


        for ht in tw_ctn_elem_ref.select(&hashtag_selector)
        {
            _ = builder.has_hashtag(ht.select(&bold_tag_selector).next().unwrap().inner_html(), 
                                    ht.attr("href").unwrap_or("").to_string())
        }

        _ = tw_ctn_elem_ref.select(&link_selector).map(|elem_ref| {
            if let Some(url) = elem_ref.attr("data-expanded-url"){
                let url = url.to_string();
            }
            else if let Some(url) = elem_ref.attr("data-resolved-url-large")
            {
                let url = url.to_string();
            }
            else {
                let url = elem_ref.inner_html();
            }

            builder.has_link(url, elem_ref.attr("href").unwrap_or("").to_string())

        });

        result.push(builder.build());

    }

    result

} 