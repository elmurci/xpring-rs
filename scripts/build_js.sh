#!/usr/bin/env bash

set -e -o pipefail

WORKING_DIR=$(pwd)

echo $WORKING_DIR

echo "Bundling JS"
cd ../js

echo ">> Installing Node Dependencies"
npm i
echo ">> Done Installing Node Dependencies"

echo "Done Bundling JS"