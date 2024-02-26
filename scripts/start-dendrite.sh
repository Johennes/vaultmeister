#!/bin/bash

root=$(git rev-parse --show-toplevel)
srcdir=$root/dendrite-src

pushd "$srcdir"

./bin/dendrite -config "$root/dendrite.yaml" -really-enable-open-registration
