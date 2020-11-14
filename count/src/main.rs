use regex::Regex;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, DirEntry},
    io,
    path::Path,
    process,
};

fn get_files(dir: String, files: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Err(e) = get_files(path.to_str().unwrap().to_string(), files) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(())
}

fn count_words(
    filename: String,
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
    // let mut table: HashMap<String, u64> = HashMap::new();
    // let filename: String = env::args().nth(1).unwrap();
    // let re = Regex::new(r"[!-/:-@\[\]`{}-~]").unwrap();
    let mut files: Vec<String> = Vec::new();

    // // TODO: Recursive call de todos os arquivos dentro do diretório
    // if let Err(e) = count_words(filename, &mut table, &re) {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }

    // let mut result: Vec<(String, u64)> = table.into_iter().collect();
    // result.sort_by(|a, b| b.1.cmp(&a.1));

    // for (name, value) in result {
    //     println!("{}: {}", name, value);
    // }

    if let Err(e) = get_files(".".to_string(), &mut files) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    println!("{:?}", files);
}
