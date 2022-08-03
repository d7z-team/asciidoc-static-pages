#!/bin/bash
if [ ! "$1" ]; then
    echo "你需要指定文档位置"
    exit 1
fi
echo '尝试安装 git 软件包'
sudo apt install git asciidoctor -y
