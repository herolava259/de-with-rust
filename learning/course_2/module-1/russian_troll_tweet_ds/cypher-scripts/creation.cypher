WITH $tweetArr AS tweets
UNWIND tweets AS tweet

MERGE (u:User {user_id: tweet.user_id})
ON CREATE SET u.screen_name = tweet.screen_name

MERGE (t: Tweet {tweet_id: tweet.tweet_id})
ON CREATE SET t.text = tweet.tweet_text,
              t.permalink = tweet.permalink

MERGE (u)-[:POSTED]->(t)

FOREACH (ht IN tweet.hashtags |
    MERGE (h:Hashtag {tag: ht.tag})
    ON CREATE SET h.archived_url = ht.archived_url
    MERGE (t)-[:HAS_TAG]->(h)
)

FOREACH (link IN tweet.links | 
  MERGE (l: Link {url: link.url})
  ON CREATE SET l.archived_url = link.archived_url
  MERGE (t)-[:HAS_TAG]->(h)
)

FOREACH (link IN tweet.links | 
  MERGE (l:Link {url: link.url})
  ON CRETE SET l.archived_url = link.archived_url
  MERGE (t)-[:HAS_LINK]->(l)
)