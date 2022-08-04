mod libs;
mod res;


use std::fs;
use std::ops::Add;
use std::path::{Path};
use std::process::Command;
use chrono::{Local, TimeZone};
use crate::file::FileInfo;
use crate::libs::config::Config;
use crate::libs::{file, git, string};
use crate::libs::prop::{PropRoot};


fn main() {
    let config = Config::load().unwrap();
    let git_file_info = git::get_all_file_commit_info(&config.project_path);
    let mut files: Vec<String> = vec![];
    let skip = &vec![&config.output_path];
    file::list_pub_files(&config.document_path, &mut files, skip);
    file::delete_dir(&(config.output_path));
    // 预处理内容替换
    let replace_attr: Vec<(String, String)> = config.attrs.iter()
        .map(|e| (String::from("{{item}}").replace("item", e.0), e.1.to_string())).collect();
    let command_attr: Vec<String> = config.attrs.iter().map(|e| format!("{}={}", e.0, e.1)).collect();
    let file_info: Vec<FileInfo> = files.iter()
        .map(|e| {
            let mut info = FileInfo::get_info(e);
            if let Some(git_info) = git_file_info.get(&info.path) {
                info.update_time = Local.timestamp_millis(git_info.last_update_time);
                info.create_time = Local.timestamp_millis(git_info.last_update_time);
                info.commit_id = git_info.last_update_commit_id.to_owned();
                info.commit_short_id = git_info.last_update_commit_short_id.to_owned();
            }
            info
        })
        .collect();
    for src_info in &file_info {
        let dist_path = file::new_path(&(config.output_path),
                                       &src_info.path.replace(&(config.document_path), ""));
        if config.attr_ext.contains(&src_info.ext) {
            // 变量填充内容
            let mut result: String = fs::read_to_string(&src_info.path).unwrap().to_string();
            string::replace_all_str(&mut result, &replace_attr);
            file::auto_write_file(&dist_path, &result);
        } else {
            // 复制其他内容
            file::auto_copy_file(&src_info.path, &dist_path);
        }
    };
    //写入 body 模板
    file::auto_write_file(&file::new_path(&config.output_path, "docinfo-footer.html"), res::res::BODY_TEMPLATE);
    // 渲染内容
    for src_info in &file_info {
        if config.doc_ext.contains(&src_info.ext) {
            let relative_dist_src_path = src_info.path.replace(&(config.document_path), "");
            let dist_src_path = file::new_path(&(config.output_path),
                                               &relative_dist_src_path);
            // 渲染 asciidoc 文档
            let dist_html_path = file::replace_file_ext(&dist_src_path, "html");
            let mut execute_command = Command::new("asciidoctor");
            for command in &command_attr {
                execute_command.arg("--attribute").arg(command);
            }
            execute_command
                .arg("--attribute").arg("nofooter")
                .arg("--attribute").arg("docinfo=shared-footer")
                .arg("--attribute").arg("toc=right")
                .arg("--attribute").arg("docinfodir=".to_string().add(&config.output_path))
                .arg("--safe-mode").arg("unsafe")
                .arg("-r").arg("asciidoctor-kroki")
                .arg("--out-file").arg(&dist_html_path).arg(dist_src_path);
            let _child = execute_command.output().expect(&format!("command {:?}  execute fail ! ", &execute_command));
            _child.status.code().expect(&format!("command error exit. command: {:?}", &execute_command));
            let mut dist_html_data = fs::read_to_string(&dist_html_path).expect(&format!("error path: {}.", &dist_html_path));
            string::replace_range(&mut dist_html_data, "{{global.source.url}}", &config.source_url);
            string::replace_range(&mut dist_html_data, "{{global.title}}", &config.info.title);
            string::replace_range(&mut dist_html_data, "{{global.home}}", &config.info.home);
            string::replace_range(&mut dist_html_data, r#".adoc">"#, r#".html">"#);
            string::replace_range(&mut dist_html_data, r#"<a href="https://"#, r#"<a target="_blank" href="https://"#);
            string::replace_range(&mut dist_html_data, r#"<a href="http://"#, r#"<a target="_blank" href="http://"#);
            string::replace_range(&mut dist_html_data, "{{file.path}}", &relative_dist_src_path);
            string::replace_range(&mut dist_html_data, "{{file.name}}", &src_info.name);
            string::replace_range(&mut dist_html_data, "{{file.commit.id}}", &src_info.commit_id);
            string::replace_range(&mut dist_html_data, "{{file.commit.short-id}}", &src_info.commit_short_id);
            string::replace_range(&mut dist_html_data, "{{file.commit.last-date}}",
                                  &src_info.update_time.format("%Y-%m-%d %H:%M:%S").to_string());
            file::auto_write_file(&dist_html_path, &dist_html_data);
        }
    }
// 编译目录
    let menu_path = file::new_path(&config.output_path, &config.location.menu);
    let mut menu_command = Command::new("asciidoctor");
    for command in &command_attr {
        menu_command.arg("--attribute").arg(command);
    }
    let menu_plain_html_path = file::replace_file_ext(&menu_path, "html.plain");
    let menu_html_path = file::new_path(&config.output_path, "menu.html");
    menu_command.arg("--no-header-footer")
        .arg("--safe-mode").arg("unsafe")
        .arg("-r").arg("asciidoctor-kroki")
        .arg("--out-file")
        .arg(&menu_plain_html_path)
        .arg(&menu_path).output().unwrap();
    let menu_plain_data = fs::read_to_string(&menu_plain_html_path).unwrap();
    let mut menu_data = res::res::MENU_TEMPLATE.to_string();
    string::replace_range(&mut menu_data, "{{body}}", &menu_plain_data);
    string::replace_range(&mut menu_data, r#".adoc">"#, r#".html">"#);
    string::replace_range(&mut menu_data, r#"<a href=""#, r#"<a target="_right_panel" href=""#);
    file::auto_write_file(&menu_html_path, &menu_data);
// 写入静态资源
    file::auto_write_file(&file::new_path(&config.output_path, "index.html"), res::res::PAGE_INDEX);
    let mut html_file: Vec<String> = Vec::new();
    file::list_pub_files(&config.output_path, &mut html_file, &Vec::new());
    let html_file: Vec<String> = html_file.iter().filter(|it| it.ends_with(".html"))
        .map(|it| it.to_string()).collect();
    let old_icon_path = &file::new_path(&config.project_path, &config.location.icon);
    let icon_name_str = Path::new(&old_icon_path).file_name().unwrap().to_str().unwrap();
    file::auto_copy_file(
        old_icon_path,
        &file::new_path(&config.output_path, icon_name_str));
// 注入默认的 css
    let mut end_data = r#"<style>{{style}}</style> <script>{{script}}</script>"#.to_string();
    let user_style_path_str = file::new_path(&config.project_path, &config.location.style);
    let user_script_path_str = file::new_path(&config.project_path, &config.location.script);
    let user_style_path = Path::new(&user_style_path_str);
    let user_script_path = Path::new(&user_script_path_str);
    let mut output_style = res::res::STYLE.to_string();
    let mut output_script = res::res::SCRIPT.to_string();
    if user_style_path.is_file() {
        output_style.push_str(&fs::read_to_string(&user_style_path).unwrap())
    }
    if user_script_path.is_file() {
        output_script.push_str(&fs::read_to_string(&user_script_path).unwrap())
    }
    string::replace_range(&mut end_data, "{{style}}", &output_style);
    string::replace_range(&mut end_data, "{{script}}", &output_script);
// 写入其他变量
    for dist_html_path in html_file {
        let mut dist_html_path_data = fs::read_to_string(&dist_html_path).unwrap();
        string::replace_range(&mut dist_html_path_data, "{{global.source.url}}", &config.source_url);
        string::replace_range(&mut dist_html_path_data, "{{global.title}}", &config.info.title);
        string::replace_range(&mut dist_html_path_data, "{{global.home}}", &config.info.home);
        string::replace_range(&mut dist_html_path_data, "{{global.main.path}}",
                              &file::replace_file_ext(&config.location.main, "html"));
        string::replace_range(&mut dist_html_path_data, "{{global.commit.id}}", "none");
        string::replace_range(&mut dist_html_path_data, "{{global.icon.path}}", icon_name_str);
        if let Some(main_git_info) = git_file_info.get(&file::new_path(&config.project_path, &config.location.main)) {
            string::replace_range(&mut dist_html_path_data, "{{global.commit.short-id}}",
                                  &main_git_info.last_update_commit_short_id);
            string::replace_range(&mut dist_html_path_data, "{{global.commit.last-date}}", &main_git_info.last_update_commit_id);
        }
        string::replace_range(&mut dist_html_path_data, "</body>", &end_data);
        file::auto_write_file(&dist_html_path, &dist_html_path_data);
    }
}
