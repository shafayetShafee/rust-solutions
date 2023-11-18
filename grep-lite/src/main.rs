use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

fn process_lines<T: BufRead + Sized>(
    reader: T, 
    re: Regex, 
    ln: bool, 
    ctx_lines: usize
    ) {
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let mut lines: Vec<String> = vec![];

    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => {
                tags.push(i);
                let v = Vec::with_capacity(2 * ctx_lines + 1);
                ctx.push(v);
            },
            None => (),
        };

        lines.push(line);
    }
    
    if tags.is_empty() {
        return;
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;
            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            if ln {
                let line_num = i + 1;
                println!("{}: {}", line_num, line);
            } else {
                println!("{}", line);
            }
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for pattern")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("line_num")
            .help("Print line number of matched line?")
            .short("l")
            .long("line-num")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("ctx_lines")
            .help("Print the context lines of the match line")
            .short("c")
            .long("ctx-lines")
            .takes_value(true)
            .required(false))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input_path = args.value_of("input").unwrap();
    let line_num = args.is_present("line_num");
    let ctx_lines = args.value_of("ctx_lines");

    let ctx_lines = match ctx_lines {
        Some(value) => value.parse::<usize>().unwrap(),
        None => 0,
    };

    let re = Regex::new(pattern).unwrap();
    let f = File::open(input_path).unwrap();
    let reader = BufReader::new(f);
    process_lines(reader, re, line_num, ctx_lines);
}
