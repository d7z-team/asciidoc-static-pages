#!/usr/bin/env bash

set -e
# 脚本工作目录
export SCRIPT_HOME=$(
    # shellcheck disable=SC2046
    cd $(dirname "${BASH_SOURCE[0]}")
    pwd
)

export LIBRARY_PATH="$SCRIPT_HOME/library"
# shellcheck disable=SC2231
for library in $LIBRARY_PATH/*.sh; do
    test -r "$library" && . "$library"
done

case "$1" in
toolchain)
    toolchain_installer
    ;;
build)
    toolchain_env
    asciidoc_build "$2"
    ;;
*)
    echo ""
    exit 1
    ;;
esac
