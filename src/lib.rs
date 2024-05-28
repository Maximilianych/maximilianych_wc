use clap::Parser;
use std::io::{Error, ErrorKind};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

/// wc from Maximilianych
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Show the size in bytes
    #[arg(short = 'c')]
    get_byte_count: bool,

    /// Show the size in lines
    #[arg(short = 'l')]
    get_line_count: bool,
    
    /// File path
    #[arg(value_name = "FILE")]
    file_path: Option<PathBuf>,
}

pub fn run(cli: Cli) {
    let result: Result<usize, Error> = match cli {
        Cli {
            get_byte_count: true,
            get_line_count: false,

            file_path: Some(ref file_path),
        } => get_byte_count(file_path),
        Cli {
            get_byte_count: false,
            get_line_count: true,

            file_path: Some(ref file_path)
        } => get_line_count(file_path),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input")),
    };

    match result {
        Ok(count) => {
            print!("{}", count);
            println!(" {}", cli.file_path.unwrap().to_str().unwrap())
        },
        Err(err) => println!("{}", err)
    }

}

pub fn get_byte_count(file_path: &PathBuf) -> Result<usize, Error> {
    Ok(file_path.metadata()?.len() as usize)
}

pub fn get_bufreader(file_path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

pub fn get_line_count(file_path: &PathBuf) -> Result<usize, Error> {
    let line_count = get_bufreader(file_path)?.lines().count(); 
    Ok(line_count)
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_count() {
        assert_eq!(342181, get_byte_count(&PathBuf::from("test.txt")).unwrap());
    }

    #[test]
    fn test_get_line_count() {
        assert_eq!(7143, get_line_count(&PathBuf::from("test.txt")).unwrap());
    }


}
