use clap::Parser;
use eyre::Result;

/// Simple program to spell words in Scrabble format
#[derive(Parser, Debug)]
#[clap(author, version = env!("GIT_DESCRIBE"), about, long_about = None)]
struct Args {
    /// The words to be spelled in Scrabble format
    words: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = args.words.join(" ");

    let scrabble_sentence = input
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .map(|c| if c.is_whitespace() {
            " ".to_string()
        } else {
            format!(":scrabble-{}:", c.to_lowercase())
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{}", scrabble_sentence);

    Ok(())
}

