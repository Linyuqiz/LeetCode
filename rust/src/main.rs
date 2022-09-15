#[path = "./leetcode/editor/cn/mod.rs"]
mod questions;

// 用于处理文档错误展示的代码段
#[allow(dead_code)]
const PLACEHOLDER: &str = "";

#[allow(dead_code)]
fn main() {
    println!("leetcode go! keep learning!");
}

#[allow(dead_code)]
const GO_FILE_SUFFIX: &str = ".go";
#[allow(dead_code)]
const RUST_FILE_SUFFIX: &str = ".rs";
#[allow(dead_code)]
const GO_PATH: &str = "/Users/axemc/Extensions/LeetCode/go/src/leetcode/editor/cn";
#[allow(dead_code)]
const RUST_PATH: &str = "/Users/axemc/Extensions/LeetCode/rust/src/leetcode/editor/cn";
#[allow(dead_code)]
const EXCLUDE_FILES: [&str; 2] = [".DS_Store", "mod.rs"];

#[test]
fn diff_file() {
    let (files, mut go_total, mut rust_total, mut sync_total) = (collect_diff_files(), 0, 0, 0);
    for v in files.iter() {
        let v = v.to_string();
        if v.ends_with(GO_FILE_SUFFIX) {
            go_total += 1;
            if !files.contains(&("_".to_owned() + &*v.replace(GO_FILE_SUFFIX, RUST_FILE_SUFFIX))) {
                sync_total += 1;
                println!("Rust: {:?}", v)
            }
        }
        if v.ends_with(RUST_FILE_SUFFIX) {
            rust_total += 1;
            if !files.contains(
                &(v.replacen("_", "", 1)
                    .replace(RUST_FILE_SUFFIX, GO_FILE_SUFFIX)),
            ) {
                sync_total += 1;
                println!("Go: {:?}", v)
            }
        }
    }
    println!(
        "Number Statistic: Go: [{}]; Rust: [{}]; Sync: [{}]",
        go_total, rust_total, sync_total
    )
}

#[allow(dead_code)]
fn collect_diff_files() -> Vec<String> {
    let mut file_list = collect_file_info(&EXCLUDE_FILES, GO_PATH);
    file_list.append(&mut collect_file_info(&EXCLUDE_FILES, RUST_PATH));
    return file_list;
}

#[allow(dead_code)]
fn collect_file_info(exclude_files: &[&str; 2], path: &str) -> Vec<String> {
    let mut file_list = vec![];
    for file in std::fs::read_dir(path).unwrap() {
        match file {
            Ok(dir) => {
                if !exclude_files.contains(&dir.file_name().to_str().unwrap())
                    && dir.file_type().unwrap().is_file()
                {
                    file_list.push(String::from(dir.file_name().to_str().unwrap()))
                }
            }
            Err(err) => panic!("{:?}", err),
        }
    }
    return file_list;
}
