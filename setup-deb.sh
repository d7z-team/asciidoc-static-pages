#!/bin/bash
if [ ! "$1" ]; then
    echo "你需要指定文档位置"
    exit 1
fi
sudo apt install git -y
git clone --depth 1 https://github.com/d7z-team/asciidoc-static-pages /tmp/doc
chmod +x /tmp/doc/doc.sh
sudo /tmp/doc/doc.sh toolchain
/tmp/doc/doc.sh build "$1"
