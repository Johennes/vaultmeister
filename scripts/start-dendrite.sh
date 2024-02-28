#!/bin/bash

root=$(git rev-parse --show-toplevel)
srcdir=$root/dendrite-src
datadir=$root/dendrite-data

pushd "$srcdir"

./bin/dendrite \
  -config "$root/dendrite.yaml" \
  -tls-cert "$datadir/server.crt" \
  -tls-key "$datadir/server.key" \
  -really-enable-open-registration
