#! /bin/sh/
wget https://github.com/fivethirtyeight/russian-troll-tweets.git
unzip russian-troll-tweets.zip
rm russian-troll-tweets.zip
mv russian-troll-tweets/* ../data/
rm -rf russian-troll-tweets.zip

echo "Russia troll tweets data downloaded and extracted to ../data/ directory."

sample_path="../data/IRAhandle_tweets_1.csv"

echo "sample data:

$(head -n 5 ../data/IRAhandle_tweets_1.csv)"


