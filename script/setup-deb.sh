#!/bin/bash
if [ ! "$1" ]; then
    echo "你需要指定文档位置"
    exit 1
fi
docker run -it --rm ghcr.io/d7z-team/asciidoc-static-pages:v2 -v $(pwd):$(pwd) -- "cd $(pwd) ; pages --output public"
