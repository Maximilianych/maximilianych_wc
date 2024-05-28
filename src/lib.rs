use clap::Parser;
use std::{io::{Error, ErrorKind}, path::PathBuf};

/// wc from Maximilianych
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Show the size in bytes
    #[arg(short = 'c')]
    get_byte_size: bool,

    /// File path
    #[arg(value_name = "FILE")]
    file_path: Option<PathBuf>,
}

pub fn run(cli: crate::Cli) {
    let result: Result<u64, Error> = match cli {
        Cli {
            get_byte_size: true,
            file_path: Some(ref file_path),
        } => get_byte_size(file_path),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input")),
    };

    match result {
        Ok(size) => {
            print!("{}", size);
            println!(" {}", cli.file_path.unwrap().to_str().unwrap())
        },
        Err(err) => println!("{}", err)
    }

}

pub fn get_byte_size(file_path: &PathBuf) -> Result<u64, Error> {
    Ok(file_path.metadata()?.len())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_size() {
        assert_eq!(342_181, get_byte_size(&PathBuf::from("test.txt")).unwrap());
    }
}
