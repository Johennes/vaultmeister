#!/bin/bash

rev=v0.13.6

root=$(git rev-parse --show-toplevel)
srcdir=$root/dendrite-src
datadir=$root/dendrite-data

announce() {
  echo -e "\033[1;35m>> $1\033[0m"
}

announce "Cloning dendrite repository"

if [[ ! -d "$srcdir" ]]; then
  git clone https://github.com/matrix-org/dendrite.git "$srcdir"
else
  echo "Already cloned"
fi

announce "Checking out $rev"

pushd "$srcdir"
git fetch
git checkout "$rev"

announce "Building dendrite"

go build -o bin/ ./cmd/...

announce "Generating private key"

if [[ ! -e "$datadir/matrix_key.pem" ]]; then
  ./bin/generate-keys --private-key "$datadir/matrix_key.pem"
else
  echo "Already generated"
fi

announce "Generating certificate"

if [[ ! -e "$datadir/server.crt" ]]; then
  ./bin/generate-keys --tls-cert "$datadir/server.crt" --tls-key "$datadir/server.key"
else
  echo "Already generated"
fi

announce "Dendrite build done"
