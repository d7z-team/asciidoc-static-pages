#!/usr/bin/env bash
# 依赖工具链安装

_root_echo() {
    echo "[root] $*"
}

__pacman_install() {
    . /etc/os-release || :
    if [ "$ID" == 'arch' ]; then
        _root_echo "操作系统确认为 ArchLinux ，将使用 pacman 安装工具链."
        INSTALL_PKG=(asciidoctor git rsync)
        for package in ${INSTALL_PKG[*]}; do
            if [ -z "$(pacman -Qs "$package")" ]; then
                _root_echo "软件包 $package 不存在，将使用 pacman 安装"
                pacman -Sy $package --noconfirm
            else
                _root_echo "软件包 $package 已存在，跳过安装"
            fi
        done

    fi

}
__apk_install() {
    . /etc/os-release || :
    INSTALL_PKG=(asciidoctor git rsync)
    for package in ${INSTALL_PKG[*]}; do
        if [ "$(apk info "$package" 2>/dev/null)" ]; then
            _root_echo "软件包 $package 不存在，将使用 apk 安装"
            apk add "$package"
        else
            _root_echo "软件包 $package 已存在，跳过安装"
        fi
    done
}
__apt_install() {
    . /etc/os-release || :
    if [ "$ID" == 'debian' ] || [ "$ID" == 'ubuntu' ]; then
        _root_echo "操作系统确认为 $NAME ，将使用 apt-get 安装工具链."
        INSTALL_PKG=(asciidoctor git rsync)
        for package in ${INSTALL_PKG[*]}; do
            if [ -z "$(dpkg -l "$package")" ]; then
                _root_echo "软件包 $package 不存在，将使用 apt-get 安装"
                apt-get update
                apt-get install -y $package
            else
                _root_echo "软件包 $package 已存在，跳过安装"
            fi
        done

    fi
}

__check_toolchain() {
    CHECK_COMMAND=(gem asciidoctor git rsync find)
    for command in ${CHECK_COMMAND[*]}; do
        # shellcheck disable=SC2086
        if [ -z "$(type $command 2>/dev/null)" ]; then
            _root_echo "软件包 $command 安装失败."
            exit 1
        fi
    done
}

__asciidoctor_extra_install() {
    gem install rouge asciidoctor-kroki || {
        _root_echo "gem 安装 rouge asciidoctor-kroki 出现错误！"
        exit 1
    }
}

_root_toolchain_installer() {
    # shellcheck disable=SC2046
    if [ -n "$(type pacman 2>/dev/null)" ]; then
        __pacman_install
    elif [ -n "$(type apt-get 2>/dev/null)" ]; then
        __apt_install
    elif [ -n "$(type apk 2>/dev/null)" ]; then
        __apk_install
    fi
    __check_toolchain
    __asciidoctor_extra_install
}

_rootless_toolchain_installer() {
    echo "普通用户暂未适配工具链安装"
}

toolchain_env() {
    export GEM_COMMAND='gem'
    export ASCIIDOCTOR_COMMAND='asciidoctor'
    export GIT_COMMAND='git'
    export RSYNC_COMMAND='rsync'
}

toolchain_installer() {
    # shellcheck disable=SC2046
    if [ $(id -u) -eq 0 ]; then
        echo "警告!将使用 root 用户安装工具链 ."
        _root_toolchain_installer
    else
        echo "将使用 $USER 用户安装工具链"
        _rootless_toolchain_installer
    fi
}
