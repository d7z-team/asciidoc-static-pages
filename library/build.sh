_build_echo() {
    echo -e "[build] $*"
}
_auto_dir() {
    # shellcheck disable=SC2068
    for dir in "$@"; do
        if [ ! -d "$dir" ]; then
            mkdir -p "$dir" 2>/dev/null || :
        fi
    done
}

asciidoc_build() {
    if [ "$1" ]; then
        if [ ! -d "$1" ]; then
            echo "directory $1 not exists."
            exit 2
        fi
        export PROJECT_HOME=$(
            cd "$1" || exit 1
            pwd .
        )
    else
        echo "Parameter error: need to specify path."
        exit 1
    fi
    # 加载 env 配置
    # shellcheck disable=SC2015
    test -r "$PROJECT_HOME/.env" && . "$PROJECT_HOME/.env" || {
        echo "未发现项目配置文件 .env ，无法继续编译 ."
        exit 1
    }
    ## 配置变量
    # 源码位置
    export SOURCE_ROOT_PATH=$(
        cd "$PROJECT_HOME" || exit 1
        pwd
    )
    # 输出位置
    export OUTPUT_ROOT_PATH=$(
        mkdir -p "$PROJECT_HOME/$DOC_OUTPUT_PATH/" >/dev/null 2>&1 || :
        cd "$PROJECT_HOME/$DOC_OUTPUT_PATH/" || exit
        pwd
    )
    git config log.date "format:%Y年%m月%d日 %H时%M分%S秒"
    # 项目地址
    export HOME_URL=$HOME_URL
    # 项目标题
    export TITLE="$DOC_TITLE"
    # 项目菜单位置
    export MENU_PATH=$DOC_MENU
    # 项目主页位置
    export MAIN_PATH=$DOC_MAIN
    # 项目图标位置
    export ICON_PATH="$DOC_ICON_FILE_PATH"
    # TEMPLATE 目录位置
    export TEMPLATE_PATH="$SCRIPT_HOME/template"
    # 静态数据位置
    export STATIC_DATA_PATH="$SCRIPT_HOME/static"
    # 远程源码查看地址
    export REMOTE_SOURCE_URL=$DOC_SOURCE_URL
    # shellcheck disable=SC2155
    export GIT_COMMIT_ID=$(
        cd "$SOURCE_ROOT_PATH" || exit
        git rev-parse HEAD || :
    )
    # shellcheck disable=SC2155
    export GIT_COMMIT_SHORT_ID=$(
        cd "$SOURCE_ROOT_PATH" || exit
        git rev-parse --short HEAD || :
    )
    # shellcheck disable=SC2155
    export DATE=$(date)

    if [ -d "$OUTPUT_ROOT_PATH" ] && [ "$OUTPUT_ROOT_PATH" != "$SOURCE_ROOT_PATH" ]; then
        /bin/rm -rf "$OUTPUT_ROOT_PATH"
    fi
    # 复制原始文件到编译目录下
    if [ "$OUTPUT_ROOT_PATH" != "$SOURCE_ROOT_PATH" ]; then
        if [ -e "/tmp/.doc-build" ]; then
            /bin/rm -rf /tmp/.doc-build
        fi
        _auto_dir "/tmp/.doc-build"
        $RSYNC_COMMAND -a --exclude '*.adoc' "$SOURCE_ROOT_PATH/"* "/tmp/.doc-build/"
        _auto_dir "$OUTPUT_ROOT_PATH"
        $RSYNC_COMMAND -a --exclude '*.adoc' "/tmp/.doc-build/"* "$OUTPUT_ROOT_PATH/" 2>/dev/null || :
    fi
    # 导入静态文件
    $RSYNC_COMMAND -a "$STATIC_DATA_PATH/"* "$OUTPUT_ROOT_PATH/"
    # shellcheck disable=SC2038
    IFS='' read -r -a DOC_FILES <<<"$(find "$SOURCE_ROOT_PATH/" -name '*.adoc' | grep -v "$OUTPUT_ROOT_PATH" | xargs echo)"
    # shellcheck disable=SC2206
    LIST_DOCS=(${DOC_FILES[@]})
    for doc in ${LIST_DOCS[*]}; do
        SRC_PATH="$doc"
        SRC_DIRECTORY="$(dirname "${SRC_PATH}")"
        SRC_FILE_NAME="$(basename "${SRC_PATH}")"
        DIST_FILE_NAME=${SRC_FILE_NAME//.adoc/.html}
        DIST_PATH=$(
            # shellcheck disable=SC2001
            echo "$SRC_DIRECTORY/$DIST_FILE_NAME" | sed "s@$SOURCE_ROOT_PATH@$OUTPUT_ROOT_PATH@g"
        )
        SRC_RELATIVE_PATH=$(
            # shellcheck disable=SC2001
            echo "$SRC_PATH" | sed "s@$SOURCE_ROOT_PATH@@g"
        )
        DIST_DIRECTORY="$(dirname "${DIST_PATH}")"
        _auto_dir "$DIST_DIRECTORY"
        GIT_CURRENT_FILE_DATE=$(
            cd "$SOURCE_ROOT_PATH"
            git log -1 --format=%cd ".$SRC_RELATIVE_PATH"
        )
        GIT_CURRENT_FILE_ID=$(
            cd "$SOURCE_ROOT_PATH"
            git log -1 --format=%H ".$SRC_RELATIVE_PATH"
        )
        GIT_CURRENT_FILE_SHORT_ID=$(
            cd "$SOURCE_ROOT_PATH"
            git log -1 --format=%h ".$SRC_RELATIVE_PATH"
        )
        $ASCIIDOCTOR_COMMAND \
            --attribute "nofooter" \
            --attribute "toc=right" \
            --attribute "docinfo=shared-footer" \
            --attribute "docinfodir=$TEMPLATE_PATH" \
            --safe-mode unsafe -r asciidoctor-kroki \
            --out-file "$DIST_PATH" "$SRC_PATH"
        sed -i -e 's/.adoc">/.html">/g' "$DIST_PATH"
        sed -i \
            -e "s@{{FILE_NAME}}@$SRC_FILE_NAME@g" \
            -e "s@{{FILE_PATH}}@$SRC_RELATIVE_PATH@g" \
            -e "s@{{FILE_DATE}}@$GIT_CURRENT_FILE_DATE@g" \
            -e "s@{{FILE_COMMIT_ID}}@$GIT_CURRENT_FILE_ID@g" \
            -e "s@{{FILE_COMMIT_SHORT_ID}}@$GIT_CURRENT_FILE_SHORT_ID@g" \
            "$DIST_PATH"
    done
    # BUILD_MENU
    MENU_HTML_OUT_PATH="$OUTPUT_ROOT_PATH/${MENU_PATH//.adoc/.html}"
    asciidoctor --safe-mode unsafe \
        --attribute "docinfo=menu" \
        --attribute "docinfodir=$TEMPLATE_PATH" \
        -r asciidoctor-kroki --no-header-footer \
        --out-file $MENU_HTML_OUT_PATH \
        "$SOURCE_ROOT_PATH/$MENU_PATH"
    sed -i 's/.adoc">/.html">/g' "$MENU_HTML_OUT_PATH"
    sed -i 's/<a href="/<a target="dist" href="/g' "$MENU_HTML_OUT_PATH"
    # BUILD HTML
    IFS='' read -r -a OUTPUT_FILES <<<"$(find "$OUTPUT_ROOT_PATH/" -name '*.html' | xargs echo)"
    # shellcheck disable=SC2206
    LIST_OUTPUT_FILES=(${OUTPUT_FILES[@]})
    for output_html in ${LIST_OUTPUT_FILES[*]}; do
        sed -i -e "s@{{SOURCE_URL}}@$REMOTE_SOURCE_URL@g" "$output_html"
        sed -i \
            -e "s@{{HOME_URL}}@$HOME_URL@g" \
            -e "s@{{COMMIT_ID}}@$GIT_COMMIT_ID@g" \
            -e "s@{{COMMIT_SHORT_ID}}@$GIT_COMMIT_SHORT_ID@g" \
            -e "s@{{BUILD_DATE}}@$DATE@g" \
            -e "s@{{TITLE}}@$TITLE@g" \
            -e "s@{{MENU_PATH}}@${MENU_PATH//.adoc/.html}@g" \
            -e "s@{{MAIN_PATH}}@${MAIN_PATH//.adoc/.html}@g" \
            -e "s@{{ICON_PATH}}@$ICON_PATH@g" \
            "$output_html"
    done
}
