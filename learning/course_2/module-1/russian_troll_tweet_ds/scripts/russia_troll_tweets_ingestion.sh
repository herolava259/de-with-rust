#! /bin/sh/
wget https://github.com/fivethirtyeight/russian-troll-tweets.git
unzip russian-troll-tweets.zip
rm russian-troll-tweets.zip
mv russian-troll-tweets/* ./
rm -rf russian-troll-tweets
