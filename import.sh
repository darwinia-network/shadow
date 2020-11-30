#!/bin/bash

shadow=./target/debug/shadow
height=15000
step=5000
geth=/Users/itering/Downloads/geth_data/geth/chaindata
mysql=mysql://root:@localhost:3306/mmr_store

to=$step
while ((to <= height))
do
  $shadow import -p $geth -u $mysql -t $to
  ((to += step))
done