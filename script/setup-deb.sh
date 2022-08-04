#!/bin/bash
if [ ! "$1" ]; then
    echo "你需要指定文档位置"
    exit 1
fi
docker run -it --rm -v $(pwd):/documents ghcr.io/d7z-team/asciidoc-static-pages:v2  pages -c /documents/.pages.yaml --output public
echo "output dir: $(pwd)/public"
