use regex::Regex;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self},
    io,
    path::PathBuf,
    process,
};

fn get_files(path: PathBuf, dir_files: &mut Vec<String>) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Err(e) = get_files(path, dir_files) {
                    eprintln!("Application error: {}", e);
                    process::exit(1);
                }
            } else {
                dir_files.push(path.to_str().unwrap().to_string());
            }
        }
    } else {
        dir_files.push(path.to_str().unwrap().to_string());
    }

    Ok(())
}

fn count_words(
    filename: &String,
    table: &mut HashMap<String, u64>,
    re: &Regex,
) -> Result<(), Box<dyn Error>> {
    let raw_content = fs::read_to_string(filename)?;
    let content = re.replace_all(&raw_content, " ");
    let words: Vec<&str> = content.split_ascii_whitespace().collect();

    for w in words {
        let count = table.entry(w.to_string()).or_insert(0);
        *count += 1;
    }

    Ok(())
}

fn main() {
    let args = env::args().skip(1);
    let mut inputs: Vec<String> = args.collect();
    let re_pontuaction = Regex::new(r"[!-/:-@\[\]`{}-~]").unwrap();
    let re_file_ext = Regex::new(r"\.(txt|js|rs|graphql|md)$").unwrap();
    let re_exclude_dir = Regex::new(r"(node_modules|target|\.gitlab|\.git)").unwrap();

    let mut table: HashMap<String, u64> = HashMap::new();
    let mut dir_files: Vec<String> = Vec::new();

    if inputs.len() == 0 {
        inputs.push(".".to_string());
    }

    for input in inputs {
        if let Err(e) = get_files(PathBuf::from(input), &mut dir_files) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }

    for file in dir_files
        .iter()
        .filter(|f| !re_exclude_dir.is_match(f))
        .filter(|f| re_file_ext.is_match(f))
        .collect::<Vec<&String>>()
    {
        if let Err(e) = count_words(file, &mut table, &re_pontuaction) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }

    let mut result: Vec<(String, u64)> = table.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, value) in result {
        println!("{}: {}", name, value);
    }
}
