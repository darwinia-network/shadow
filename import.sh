#!/bin/bash

shadow=./target/debug/shadow
height=20000
step=5000
geth=/Users/itering/Downloads/geth_data/geth/chaindata
mysql=mysql://root:@localhost:3306/mmr_store

count=$($shadow count -u $mysql | cut -d":" -f2)
to=$((count+step))
while ((to <= height))
do
  $shadow import -p $geth -u $mysql -t $to
  ((to += step))
done