use clap::Parser;
use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, Error, Read};
use std::path::PathBuf;
use std::process;

/// wc from Maximilianych
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Show the size in bytes (characters)
    #[arg(short = 'c')]
    get_byte_count: bool,

    /// Show the size in lines
    #[arg(short = 'l')]
    get_line_count: bool,

    /// Show the size in words
    #[arg(short = 'w')]
    get_word_count: bool,

    /// Show the size in characters
    #[arg(short = 'm')]
    get_char_count: bool,

    /// Files path
    #[arg(value_name = "FILES", num_args(0..))]
    file_path: Vec<PathBuf>,
}

pub fn run(cli: Cli) {
    let mut result_vec = Vec::new();

    if cli.file_path.len() >= 1 {
        for path in &cli.file_path {
            let mut file = String::new();
            File::open(path)
                .unwrap_or_else(|e| {
                    eprintln!("Error: {e}");
                    process::exit(1);
                })
                .read_to_string(&mut file)
                .unwrap_or_else(|e| {
                    eprintln!("Error: {e}");
                    process::exit(1);
                });
            let result = get_for_file(&cli, file);
            result_vec.push(result);
        }
    } else if cli.file_path.len() < 1 {
        let mut stdin = io::stdin();
        let mut file = String::new();
        stdin.read_to_string(&mut file).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            process::exit(1);
        });
        let result = get_for_file(&cli, file);
        result_vec.push(result);
    }

    let path_iter = &mut cli.file_path.iter();
    result_vec.iter().for_each(|res| {
        res.iter().for_each(|(_, v)| print!("{v}\t"));
        if let Some(path) = path_iter.next() {
            print!("{}", path.to_str().unwrap());
        }
        print!("\n")
    });
    result_vec
        .iter()
        .next()
        .unwrap()
        .iter()
        .for_each(|(k, _)| print!("{k}\t"));
}

pub fn get_for_file(cli: &Cli, file: String) -> IndexMap<String, usize> {
    let mut result = IndexMap::new();
    if cli.get_byte_count {
        if let Ok(byte) = get_byte_count(&file) {
            result.insert("Bytes".to_string(), byte);
        }
    }

    if cli.get_char_count {
        if let Ok(char) = get_char_count(&file) {
            result.insert("Chars".to_string(), char);
        }
    }

    if cli.get_word_count {
        if let Ok(word) = get_word_count(&file) {
            result.insert("Words".to_string(), word);
        }
    }

    if cli.get_line_count {
        if let Ok(line) = get_line_count(&file) {
            result.insert("Lines".to_string(), line);
        }
    }

    if !cli.get_byte_count && !cli.get_char_count && !cli.get_word_count && !cli.get_line_count {
        if let Ok((byte, char, word, line)) = get_all_count(&file) {
            result.insert("Bytes".to_string(), byte);
            result.insert("Chars".to_string(), char);
            result.insert("Words".to_string(), word);
            result.insert("Lines".to_string(), line);
        }
    }
    result
}

pub fn get_byte_count(file: &String) -> Result<usize, Error> {
    Ok(file.len())
}

pub fn get_char_count(file: &String) -> Result<usize, Error> {
    Ok(file
        .chars()
        .filter(|x| !(*x == '\n') && !(*x == '\r'))
        .count())
}

pub fn get_word_count(file: &String) -> Result<usize, Error> {
    Ok(file.split_whitespace().count())
}

pub fn get_line_count(file: &String) -> Result<usize, Error> {
    Ok(file.split('\n').count())
}

pub fn get_all_count(file: &String) -> Result<(usize, usize, usize, usize), Error> {
    let bc = file.len();
    let cc = file
        .chars()
        .filter(|x| !(*x == '\n') && !(*x == '\r'))
        .count();
    let wc = file.split_whitespace().count();
    let lc = file.split('\n').count();

    Ok((bc, cc, wc, lc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_count() {
        let mut file = String::new();
        File::open("test.txt")
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();
        assert_eq!(342181, get_byte_count(&file).unwrap());
    }

    #[test]
    fn test_get_char_count() {
        let mut file = String::new();
        File::open("test.txt")
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();
        assert_eq!(325001, get_char_count(&file).unwrap());
    }

    #[test]
    fn test_get_word_count() {
        let mut file = String::new();
        File::open("test.txt")
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();
        assert_eq!(58164, get_word_count(&file).unwrap());
    }

    #[test]
    fn test_get_line_count() {
        let mut file = String::new();
        File::open("test.txt")
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();
        assert_eq!(7143, get_line_count(&file).unwrap());
    }

    #[test]
    fn test_get_all_count() {
        let mut file = String::new();
        File::open("test.txt")
            .unwrap()
            .read_to_string(&mut file)
            .unwrap();
        assert_eq!((342181, 325001, 58164, 7143), get_all_count(&file).unwrap());
    }
}
