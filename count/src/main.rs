use regex::Regex;
use std::{collections::HashMap, env, error::Error, fs, process};

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

    return Ok(());
}

fn main() {
    let mut table: HashMap<String, u64> = HashMap::new();
    let filename: String = env::args().nth(1).unwrap();
    let re = Regex::new(r"[!-/:-@\[\]`{}-~]").unwrap();

    // TODO: Recursive call de todos os arquivos dentro do diretório
    if let Err(e) = count_words(filename, &mut table, &re) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    let mut result: Vec<(String, u64)> = table.into_iter().collect();
    result.sort_by(|a, b| a.1.cmp(&b.1));

    for (name, value) in result {
        println!("{}: {}", name, value);
    }
}
