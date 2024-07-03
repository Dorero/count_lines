use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No arguments passed. You need to pass the path to the directory.")
    }

    let mut table: HashMap<String, usize> = HashMap::new();

    if args.len() > 2 {
        for arg in &args[2..] {
            table.insert(arg.to_string(), 0);
        }
    }

    parse_dir(&mut table, args[1].clone(), &args[2..]);

    println!("{:?}", table)
}

pub fn parse_dir(table: &mut HashMap<String, usize>, path: String, extentions: &[String]) {
    let metadata = fs::metadata(&path).expect("Filed to get info about file/dir");
    if metadata.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            parse_dir(
                                table,
                                entry
                                    .path()
                                    .to_str()
                                    .expect("Filed to convert file path")
                                    .to_string(),
                                extentions,
                            );
                        }
                        Err(e) => eprintln!("Failed to read record: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory: {}", e),
        }
    } else if metadata.is_file() {
        count_lines(table, path, extentions);
    } else {
        panic!("It's not file or directory. Please, enter file or directory.")
    }
}

pub fn count_lines(table: &mut HashMap<String, usize>, path: String, extentions: &[String]) {
    let extention = path
        .split(".")
        .last()
        .expect("Filed to find file extention")
        .to_string();

    if extentions.len() > 0 && !extentions.contains(&extention) {
        return;
    }

    let mut file = fs::File::open(&path).expect("Filed to open file");
    let mut content: Vec<u8> = Vec::new();

    match file.read_to_end(&mut content) {
        Ok(_) => match String::from_utf8(content) {
            Ok(valid_string) => {
                let lines = valid_string.lines().count();
                if extentions.len() > 0 {
                    if let Some(ext) = table.get_mut(&extention) {
                        *ext += lines;
                    }
                } else {
                    table.insert(extention, lines);
                }
            }
            Err(_) => (),
        },
        Err(e) => {
            eprintln!("Failed to read from file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_count_lines() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("test.txt");

        let mut file = File::create(&file_path).expect("Failed to create file");
        writeln!(file, "Line 1\nLine 2\nLine 3").expect("Failed to write to file");

        let mut table: HashMap<String, usize> = HashMap::new();

        count_lines(&mut table, file_path.to_str().unwrap().to_string(), &[]);

        assert_eq!(table.get("txt").cloned().unwrap_or(0), 3);
    }

    #[test]
    fn test_parse_dir() {
        let dir = tempdir().expect("Failed to create temp dir");

        let file_path1 = dir.path().join("test1.txt");
        let file_path2 = dir.path().join("test2.rs");

        let mut file1 = File::create(&file_path1).expect("Failed to create file1");
        let mut file2 = File::create(&file_path2).expect("Failed to create file2");

        writeln!(file1, "Line 1\nLine 2\nLine 3").expect("Failed to write to file1");
        writeln!(file2, "Line 1\nLine 2").expect("Failed to write to file2");

        let mut table: HashMap<String, usize> = HashMap::new();
        table.insert("txt".to_string(), 0);
        table.insert("rs".to_string(), 0);

        parse_dir(
            &mut table,
            dir.path().to_str().unwrap().to_string(),
            &["txt".to_string(), "rs".to_string()],
        );

        assert_eq!(table.get("txt").cloned().unwrap_or(0), 3);
        assert_eq!(table.get("rs").cloned().unwrap_or(0), 2);
    }
}
