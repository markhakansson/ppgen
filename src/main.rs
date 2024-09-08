use clap::Parser;
use rand::Rng;
use std::{
    fs,
    path::{Path, PathBuf},
};

const DEFAULT_WORDLIST: &str = "/usr/share/dict/words";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of words to use.
    #[arg(short, long)]
    count: usize,
    /// Optional separator to use between words.
    #[arg(long)]
    word_separator: Option<String>,
    /// Optional wordlist to use.
    #[arg(short, long)]
    wordlist: Option<PathBuf>,
}

struct PwOptions {
    count: usize,
    word_separator: String,
}

fn read_worldlist(path: PathBuf) -> Vec<String> {
    let contents = fs::read_to_string(&path).expect("should have been able to read file");
    let res: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    res
}

fn generate_passphrase(wordlist: &Vec<String>, options: &PwOptions) -> String {
    let indexes = randomize_indexes(options.count, wordlist.len());
    let mut passphrase = String::new();

    for (i, index) in indexes.iter().enumerate() {
        match wordlist.get(*index) {
            Some(s) => {
                let string: String = if i == options.count - 1 {
                    s.to_string()
                } else {
                    format!("{s}{}", options.word_separator)
                };

                passphrase.push_str(&string);
            }
            None => panic!("accessing value that does not exist"),
        }
    }

    passphrase
}

fn randomize_indexes(count: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut indexes: Vec<usize> = Vec::with_capacity(count);

    for _ in 0..count {
        let num = rng.gen_range(0..max);
        indexes.push(num);
    }

    indexes
}

fn get_pw_options(args: &Args) -> PwOptions {
    let word_separator = match &args.word_separator {
        Some(s) => s.clone(),
        None => "".to_string(),
    };
    PwOptions {
        count: args.count,
        word_separator,
    }
}

fn main() {
    let args = Args::parse();

    let options = get_pw_options(&args);
    let wordlist_path = match args.wordlist {
        Some(path) => path,
        None => Path::new(DEFAULT_WORDLIST).to_path_buf(),
    };

    let wordlist = read_worldlist(wordlist_path);
    let pass = generate_passphrase(&wordlist, &options);

    println!("{pass}");
}
