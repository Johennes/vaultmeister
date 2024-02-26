#!/bin/bash

root=$(git rev-parse --show-toplevel)
srcdir=$root/dendrite-src

pushd "$srcdir"

./bin/create-account -config "$root/dendrite.yaml" -username "$1"
