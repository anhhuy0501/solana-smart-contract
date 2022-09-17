#!/bin/bash

rm -r dist/
bash ./run.sh deploy

bash ./run.sh client &

sleep 2
bash swap.sh

