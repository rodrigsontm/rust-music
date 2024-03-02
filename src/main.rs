use std::fs;

use clap::Parser;
use serde::Deserialize;


fn main() {

    let args = Args::parse();
    let contents = fs::read_to_string(args.settings).expect("File does not exists, input a valid path");
    let settings: AuthorsToDownload = serde_json::from_str(&contents).unwrap();
    for author in settings.authors {
        print!("Author name {}\n", &author.name);
        print!("First {} is {}", &author.obras[0].tipo, &author.obras[0].name)
    }
}

#[derive(Parser, Debug)]
struct Args {

    /// Path to config file
    #[arg(short, long, default_value_t = ("setting.json").to_string())]
    settings: String,
}

#[derive(Deserialize, Debug)]
struct AuthorsToDownload {
    authors: Vec<AuthorOptions>,
}

#[derive(Deserialize, Debug)]
struct AuthorOptions {
    name: String,
    obras: Vec<Obra>
}

#[derive(Deserialize, Debug)]
struct Obra {
    name: String,
    tipo: String,
}