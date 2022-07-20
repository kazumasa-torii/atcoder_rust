use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(render: T, re: Regex) {
    for line_ in render.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

pub fn main() {
    let args = App::new("grep-lite") // コマンド引数のパーサを徐々に構築する
        .version("0.1") // 個々の引数が Arg を 1 つ取るが、
        .about("searches for patterns") // この場合に必要な引数は 1 個だけ
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("input")
            .help("file to search")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    // パターン引数を抽出する
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    
    if input == "-" {
        let stdin = io::stdin();
        let render = stdin.lock();

        process_lines(render, re);
    } else {
        let f = File::open(input).unwrap();
        let render = BufReader::new(f);
        process_lines(render, re);
    }

}
