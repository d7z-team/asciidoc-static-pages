#!/bin/bash
if [ ! "$1" ]; then
    echo "你需要指定文档位置"
    exit 1
fi
echo '尝试安装 git 软件包'
sudo apt install git -y
echo '克隆 d7z-team/asciidoc-static-pages 项目'
git clone --depth 1 https://github.com/d7z-team/asciidoc-static-pages /tmp/doc
chmod +x /tmp/doc/doc.sh
echo '安装工具链'
sudo /tmp/doc/doc.sh toolchain
echo '渲染文档'
/tmp/doc/doc.sh build "$1"
echo '文档创建完成'
