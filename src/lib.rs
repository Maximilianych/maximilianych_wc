use clap::Parser;
use std::borrow::Cow;
use std::io::{Error, Read};
use std::fs::File;
use std::path::PathBuf;
use encoding_rs::UTF_8;
use indexmap::IndexMap;

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
    
    /// Show the size in lines
    #[arg(short = 'w')]
    get_word_count: bool,

    /// Show the size in characters
    #[arg(short = 'm')]
    get_char_count: bool,

    /// File path
    #[arg(value_name = "FILE")]
    file_path: PathBuf,
}

pub fn run(cli: Cli) {
    let mut result = IndexMap::new();
    let file_path = cli.file_path;

    if cli.get_byte_count {
        if let Ok(byte) = get_byte_count(&file_path) {
            result.insert("Bytes", byte);
        }
    }

    if cli.get_char_count {
        if let Ok(char) = get_char_count(&file_path) {
            result.insert("Chars", char);
        }
    }

    if cli.get_word_count {
        if let Ok(word) = get_word_count(&file_path) {
            result.insert("Words", word);
        }
    }

    if cli.get_line_count {
        if let Ok(line) = get_line_count(&file_path) {
            result.insert("Lines", line);
        }
    }

    if !cli.get_byte_count && !cli.get_char_count && !cli.get_word_count && !cli.get_line_count {
        if let Ok((byte, char, word, line)) = get_data(&file_path) {
            result.insert("Byte", byte);
            result.insert("Chars", char);
            result.insert("Words", word);
            result.insert("Lines", line);
        }
    }

    for key in result.keys() {
        print!("{}\t", result.get(key).unwrap());
    }
    print!("{}\n", file_path.to_str().unwrap());
    for key in result.keys() {
        print!("{}\t", key);
    }
}

pub fn get_byte_count(file_path: &PathBuf) -> Result<usize, Error> {
    Ok(file_path.metadata()?.len() as usize)
}

pub fn get_decoded_file<'a>(file_path: &PathBuf, buf: &'a mut Vec<u8>) -> Result<Cow<'a, str>, Error> {
    File::open(file_path)?.read_to_end(buf)?;
    let file = UTF_8.decode(buf);

    Ok(file.0)
}

pub fn get_char_count(file_path: &PathBuf) -> Result<usize, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let file = get_decoded_file(file_path, &mut buf)?;

    Ok(file.chars().filter(|x| !(*x=='\n') && !(*x=='\r')).count())
}

pub fn get_word_count(file_path: &PathBuf) -> Result<usize, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let file = get_decoded_file(file_path, &mut buf)?;

    Ok(file.split_whitespace().count())
}

pub fn get_line_count(file_path: &PathBuf) -> Result<usize, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let file = get_decoded_file(file_path, &mut buf)?;

    Ok(file.split('\n').count())
}

pub fn get_data(file_path: &PathBuf) -> Result<(usize, usize, usize, usize), Error> {
    let mut buf: Vec<u8> = Vec::new();
    let file = get_decoded_file(file_path, &mut buf)?;

    let bc = file_path.metadata()?.len() as usize;
    let cc = file.chars().filter(|x| !(*x=='\n') && !(*x=='\r')).count();
    let wc = file.split_whitespace().count();
    let lc = file.split('\n').count();

    Ok((bc, cc, wc, lc))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_count() {
        assert_eq!(342181, get_byte_count(&PathBuf::from("test.txt")).unwrap());
    }

    #[test]
    fn test_get_char_count() {
        assert_eq!(325001, get_char_count(&PathBuf::from("test.txt")).unwrap());

    }
    
    #[test]
    fn test_get_word_count() {
        assert_eq!(58164, get_word_count(&PathBuf::from("test.txt")).unwrap());
    }
    
    #[test]
    fn test_get_line_count() {
        assert_eq!(7143, get_line_count(&PathBuf::from("test.txt")).unwrap());
    }

    #[test]
    fn test_get_data() {
        assert_eq!((342181, 325001, 58164, 7143), get_data(&PathBuf::from("test.txt")).unwrap());
    }

}
