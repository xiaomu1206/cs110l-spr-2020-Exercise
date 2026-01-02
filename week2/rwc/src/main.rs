use std::env;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::process;

pub struct WordCount {
    words:usize,
    lines:usize,
    bytes:usize,
}

impl WordCount {
    pub fn new(words_cnt:usize, lines_cnt:usize, bytes_cnt: usize) -> WordCount{
        WordCount {
            words : words_cnt,
            lines : lines_cnt,
            bytes : bytes_cnt,
        }
    }

    pub fn display(&self) {
        println!("words_cnt:{}, lines_cnt:{}, bytes_cnt:{}", self.words, self.lines, self.bytes);
    }
}

fn read_file_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut lines: Vec<String> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        lines.push(line_str);
    }

    Ok(lines)
}

fn rwc(lines: &Vec<String>) -> WordCount {
    let mut words_cnt = 0;
    let mut bytes_cnt = 0;
    let lines_cnt = lines.len();

    for line in lines {
        words_cnt += line.split_whitespace().count();
        bytes_cnt += line.as_bytes().len();
    }

    WordCount::new(words_cnt, lines_cnt, bytes_cnt)
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }

    let filename = &args[1];
    let lines = read_file_lines(filename)?;
    let wordcount = rwc(&lines);

    wordcount.display();
    Ok(())
}
