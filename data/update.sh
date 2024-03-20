#!/usr/bin/env bash

set -xueo pipefail

curl \
  --user-agent "okhttp/3.12.1" \
  'https://metro-rti.nexus.org.uk/api/stations' > './station_names.json'

curl \
  --user-agent "okhttp/3.12.1" \
  'https://metro-rti.nexus.org.uk/api/stations/platforms' > './platforms.json'
