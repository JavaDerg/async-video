#!/bin/sh

if ! type "cargo" 2>&1 /dev/null 2>&1; then
  echo "cargo command not found"
  while true; do
    read -rp "Do you wish to install rust from rustup.rs? y/n" yn
    case $yn in
        [Yy]* ) curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; break;;
        [Nn]* ) exit;;
        * ) echo "Please answer yes or no.";;
    esac
  done
fi
cargo fetch # fetch rust dependencies

pushd js
if ! type "yarn" 2>&1 /dev/null 2>&1; then
  echo "yarn not found"
  while true; do
    read -rp "Do you wish to install yarn from npm? y/n" yn
    case $yn in
        [Yy]* ) sudo npm i -g yarn; break;;
        [Nn]* ) exit;;
        * ) echo "Please answer yes or no.";;
    esac
  done
fi
yarn
popd

touch .setup