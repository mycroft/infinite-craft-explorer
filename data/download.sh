#!/bin/sh

curl -LO https://infinite-craft.gg/recipes/data/index.json


for i in {1000..2200}
do
  curl -LO https://infinite-craft.gg/recipes/data/chunks/chunk-${i}.json
done
