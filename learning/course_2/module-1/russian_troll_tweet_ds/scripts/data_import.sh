#! /bin/bash/

# take containerId as from docker running

neo4j_container_id=$(docker ps -qf "name=$1")

if [ "$#" -gt 2 ]; then
    tweet_ds_source="$3"
else
    tweet_ds_source="https://github.com/fivethirtyeight/russian-troll-tweets.git"
fi

echo "Downloading Russia troll tweets data from $tweet_ds_source"

wget $tweet_ds_source -P ../data_source/ $tweet_ds_source

unzip ../data_source

