#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

docker build --target release . -t ogontaro/image-mirror:latest
docker push ogontaro/image-mirror:latest
