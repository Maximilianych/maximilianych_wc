## About the project
My attempt to create a `wc` tool that can count the number of bytes, characters, words and lines in one and multiple files, and from standard input.<br>
I'm pretty sure it works a bit wrong, but I'm tired of figuring out the problem too, so let it stay that way.<br>
I don't recommend using it.
## What I used
Everything was written in `Rust` using [Clap](https://crates.io/crates/clap), a great thing that allows you to work with the command line and attributes easily and conveniently.
## Installation
Install `Rust`, and type the following command into the console:
```
> cargo install --git https://github.com/Maximilianych/maximilianych_wc.git
```
You can download the source code and install using: 
```
> cargo install --path .\
```
from the project folder.
## How to use
After installation, use the `mwc` command in the console.<br>
`mwc -h` will show a description and possible attributes.
```
> mwc -h

Usage: mwc.exe [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Files path

Options:
  -c             Show the size in bytes (characters)
  -l             Show the size in lines
  -w             Show the size in words
  -m             Show the size in characters
  -h, --help     Print help
  -V, --version  Print version
```
You can use one attribute at a time, several attributes at a time or no attributes, in the last case everything will be displayed.
```
> mwc -l test.txt
7143    test.txt
Lines
```
```
> mwc -l -c  test.txt
342181  7143    test.txt
Bytes   Lines
```
```
> mwc test.txt  
342181  325001  58164   7143    test.txt
Bytes   Chars   Words   Lines
```
Can be used with one or more files, you can't enter a directory, I didn't do that.<br>
It is possible not to enter file paths, then the input will come from standard input, where you can enter the text yourself or using, for example, `cat`.
```
> mwc test.txt test2.txt  
342181  325001  58164   7143    test.txt
339178  322159  57655   7076    test2.txt
Bytes   Chars   Words   Lines
```
```
> cat test.txt | mwc
342183  327897  58164   7144
Bytes   Chars   Words   Lines
```
```
> cat test.txt | mwc -w -m
327897  58164
Chars   Words
```
When using `cat` the output is slightly different, I haven't figured out why yet.<br>
At the end an empty string is added, which increases `Lines` and `Bytes` by 1 and 2 respectively, and some characters, like those that take more than two bytes, are split into several (`™` turns into `v “ў`), how to fix it I haven't figured it out yet, maybe I'll fix it later.
