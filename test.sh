#!/bin/bash

# This script is constantly requesting the response from the api

i="0"

while true; do
echo $(curl localhost:8080/$i)
i=$[i+1]
done