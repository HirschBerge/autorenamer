use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::ffi::OsStr;
// use std::fmt::format;
use std::path::Path;
use std::{env, fs};
#[derive(Debug, Parser)]
#[clap(name = "Autorenamer", version = "1.0.3", author = "HirschBerge")]

pub struct Autorename {
    #[clap(long = "season", short = 's')]
    season: i32,
    #[clap(long = "path", short = 'p', required = false)]
    path: Option<String>,
}
#[derive(Debug)]
struct Episode {
    old_path: String,
    new_path: String,
}
impl Episode {
    fn new(old_path: String, new_path: String) -> Episode {
        Episode { old_path, new_path }
    }
    fn create_ext(&self) -> String {
        return Path::new(&self.old_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("mp4")
            .to_string();
    }
    fn create_new_path(&self, base_path: String, ext: String, file: String) -> String {
        println!(
            "\x1b[31m{}\x1b[0m => \x1b[35m{}.{}\x1b[0m",
            file, &self.new_path, ext
        );
        format!("{}/{}.{}", base_path, &self.new_path, ext)
    }
}
fn get_episodes(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut matching_files: Vec<String> = Vec::new();
    if let Ok(files) = fs::read_dir(path) {
        for file in files {
            if let Ok(file) = file {
                let path = file.path();
                if path.is_file() {
                    if path
                        .file_name()
                        .ok_or("Invalid File Name")?
                        .to_string_lossy()
                        .to_string()
                        .contains("Episode")
                    {
                        matching_files.push(
                            path.file_name()
                                .ok_or("Invalid")?
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            }
        }
        Ok(matching_files)
    } else {
        Err("Failed to read the directory".into())
    }
}
fn rename_episodes(files: Result<Vec<String>, Box<dyn Error>>, season: i32, base_path: String) {
    match files {
        Ok(files) => {
            for file in files {
                let re = Regex::new(r"Episode \d+").unwrap();
                if let Some(captures) = re.captures(&file) {
                    if let Some(matched_str) = captures.get(0) {
                        let new_name =
                            format!("S{:0>2}E{:0>2}", season, &matched_str.as_str()[8..]);
                        let old_name = format!("{}/{}", base_path, file);
                        let episode = Episode::new(old_name, new_name);
                        let _ = fs::rename(
                            &episode.old_path,
                            episode.create_new_path(base_path.clone(), episode.create_ext(), file),
                        );
                    }
                } else {
                    println!("Pattern not found in the input text.");
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn main() {
    let args = Autorename::parse();
    let mut path: String = args.path.unwrap_or_else(|| String::from("")).to_string();
    if path.is_empty() {
        path = env::current_dir().unwrap().to_string_lossy().to_string();
    }
    let season = args.season;
    let result = get_episodes(path.clone());
    rename_episodes(result, season, path);
}
