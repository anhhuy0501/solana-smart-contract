#!/bin/bash

curl --location --request POST 'http://localhost:5050/swap' \
     --header 'Content-Type: application/json' \
     --data-raw '{
         "amount": 123
     }'