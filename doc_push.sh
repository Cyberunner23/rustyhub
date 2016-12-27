#!/bin/sh

# License: CC0 1.0 Universal
# https://creativecommons.org/publicdomain/zero/1.0/legalcode

set -e

cargo doc

mkdir -p ~/.ssh
openssl aes-256-cbc -K $encrypted_d9ac7fa35307_key -iv $encrypted_d9ac7fa35307_iv -in ./id_rsa.enc -out ~/.ssh/id_rsa -d
chmod 600 ~/.ssh/id_rsa

git clone --branch gh-pages git@github.com:Cyberunner23/rustyhub deploy_docs

cd deploy_docs
git config user.name "doc upload bot"
git config user.email "nobody@example.com"
rm -rf rustyhub
mv ../target/doc rustyhub
git add -A rustyhub
git commit -qm "Doc upload."
git push -q origin gh-pages