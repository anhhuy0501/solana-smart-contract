#!/bin/bash

# Nodejs
cd ~
curl -sL https://deb.nodesource.com/setup_16.x -o /tmp/nodesource_setup.sh
sudo bash /tmp/nodesource_setup.
sudo apt install nodejs
node -

# Yarn
sudo corepack enable
yarn set version stable

# Anchor
cargo install --git https://github.com/project-serum/anchor avm --locked --force
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev

avm install latest
avm use latest
anchor --version
