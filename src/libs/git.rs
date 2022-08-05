use std::collections::HashMap;
use git2::{Repository};
use crate::file;

#[derive(Debug)]
pub struct GitFileInfo {
    pub path: String,
    pub last_update_time: i64,
    pub last_update_commit_id: String,
    pub last_update_commit_short_id: String,
}

pub fn get_all_file_commit_info(git_root_path: &str) -> HashMap<String, GitFileInfo> {
    let mut map: HashMap<String, GitFileInfo> = HashMap::new();
    let repository = Repository::open(git_root_path).unwrap();
    let mut rev_walk = repository.revwalk().unwrap();
    rev_walk.set_sorting(git2::Sort::TIME).unwrap();
    rev_walk.push_head().unwrap();
    for commit_id in rev_walk {
        let commit_id = match commit_id {
            Ok(it) => it,
            Err(_) => continue,
        };
        let commit = repository.find_commit(commit_id).unwrap();
        if commit.parent_count() == 1 {
            let prev_commit = commit.parent(0).unwrap();
            let tree = commit.tree().unwrap();
            let prev_tree = prev_commit.tree().unwrap();
            let diff = repository.diff_tree_to_tree(Some(&prev_tree), Some(&tree), None).unwrap();
            for delta in diff.deltas() {
                let path = file::new_path(git_root_path, delta.new_file().path().unwrap().to_str().unwrap());
                let current = GitFileInfo {
                    path: path.to_string(),
                    last_update_time: (commit.time().seconds() * 1000),
                    last_update_commit_id: commit_id.to_string(),
                    last_update_commit_short_id: commit_id.to_string().split_at(6).0.to_owned(),
                };
                if let Some(data) = map.get_mut(&path) {
                    if data.last_update_time < current.last_update_time {
                        *data = current
                    }
                } else {
                    map.insert(path.to_owned(), current);
                }
            }
        }
    }
    return map;
}
