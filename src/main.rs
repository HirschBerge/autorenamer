use autorenamer::file_ops::{get_episodes, rename_episodes};
use clap::Parser;
use std::env;

#[derive(Debug, Parser)]
#[clap(name = "Autorenamer", version = "1.0.3", author = "HirschBerge")]
pub struct Autorename {
    #[clap(
        long = "season",
        short = 's',
        help = "The season of the show to rename."
    )]
    season: i32,
    #[clap(
        long = "path",
        short = 'p',
        required = false,
        help = "A valid path to the directory containing your season episodes."
    )]
    path: Option<String>,
    #[clap(
        long = "offset",
        short = 'o',
        required = false,
        allow_hyphen_values = true,
        help = "An integer with a positive or negative number to offset renaming by. i.e. --ofset 5 changes 'Episode 5.mp4' to 'Episode 10.mp4'"
    )] // HACK: allow_hyphen_values just lets this take negative values
    offset: Option<i32>,
    #[arg(
        long = "dryrun",
        short = 'd',
        help = "Shows the 'whatif' events without actually writing changes to disk"
    )]
    dryrun: bool,
}

fn main() {
    let args = Autorename::parse();
    let dryrun: bool = args.dryrun;
    let mut path: String = args.path.unwrap_or_else(|| String::from("")).to_string();
    if path.is_empty() {
        path = env::current_dir()
            .expect("Expected PWD to be real. Not sure how this happened.")
            .to_string_lossy()
            .to_string();
    }
    let offset = args.offset.unwrap_or(0);
    let season = args.season;
    let result = get_episodes(path.clone());
    match result {
        Ok(result) => {
            rename_episodes(result, season, path, offset, dryrun, true);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
