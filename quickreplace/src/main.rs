use regex::Regex;
use std::{env, fs, process};
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    filename: String,
    output: String,
    replacement: String,
    target: String,
}

fn main() {
    let args = parse_args();

    fs::read_to_string(&args.filename)
        .map_err(|e| {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            process::exit(1);
        })
        .and_then(|data| replace(&args.target, &args.replacement, &data))
        .map_err(|e| {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            process::exit(1);
        })
        .and_then(|data| fs::write(&args.output, &data))
        .map_err(|e| {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            process::exit(1);
        })
        .unwrap();
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error:".red().bold(),
            args.len()
        );
        process::exit(1);
    }

    Arguments {
        filename: args[2].clone(),
        output: args[3].clone(),
        replacement: args[1].clone(),
        target: args[0].clone(),
    }
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );

    eprintln!("Usage: quickreplace <target> <replacement> <input> <output>");
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    Regex::new(target).map(|regex| regex.replace_all(text, replacement).to_string())
}
