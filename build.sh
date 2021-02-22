#!/bin/sh

if ! [ -f ".setup" ]; then
  echo "Running setup..."
  ./setup.sh
fi

pushd js
yarn build
popd
rm assets/* -rf
cp js/dist/* "assets" -r
cargo build --release