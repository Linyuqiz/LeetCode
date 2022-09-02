mod leetcode;

// 用于处理文档错误展示的代码段
#[allow(dead_code)]
const PLACEHOLDER: &str = "";

#[allow(dead_code)]
fn main() {
    println!("leetcode");
}

#[test]
fn diff_file() {
    let (go_file_suffix, rust_file_suffix) = (".go", ".rs");
    let (files, mut go_total, mut rust_total, mut sync_total) = (collect_diff_files(), 0, 0, 0);
    for v in files.iter() {
        let v = v.to_string();
        if v.ends_with(go_file_suffix) {
            go_total += 1;
            if !files.contains(&("_".to_owned() + &*v.replace(go_file_suffix, rust_file_suffix))) {
                sync_total += 1;
                println!("Rust: {:?}", v)
            }
        }
        if v.ends_with(rust_file_suffix) {
            rust_total += 1;
            if !files.contains(
                &(v.replacen("_", "", 1)
                    .replace(rust_file_suffix, go_file_suffix)),
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
    let (go_path, rust_path) = (
        "/Users/axemc/Extensions/LeetCode/go/src/leetcode/editor/cn",
        "/Users/axemc/Extensions/LeetCode/rust/src/leetcode/editor/cn",
    );
    let exclude_files = vec![".DS_Store", "mod.rs"];
    let mut file_list = collect_file_info(&exclude_files, go_path);
    file_list.append(&mut collect_file_info(&exclude_files, rust_path));
    return file_list;
}

#[allow(dead_code)]
fn collect_file_info(exclude_files: &Vec<&str>, path: &str) -> Vec<String> {
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
