#!/bin/bash

# agg=agg
agg=(docker run --rm -it -u "$(id -u):$(id -g)" -v "$PWD:/data" lpenz/agg:1.4.3)

set -e -x

for demofile in ./demos/*.py; do
    name="${demofile##*/}"
    name="${name//.py/}"
    asciinema rec --overwrite \
        --rows 25 --cols 100 \
        -c "$demofile" "demos/${name}.cast"
    "${agg[@]}" \
        --speed 1 \
        --theme asciinema \
        "demos/${name}.cast" "demos/${name}.gif"
done
