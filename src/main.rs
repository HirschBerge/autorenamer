use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fmt::format;
use std::{env, fs};

#[derive(Debug, Parser)]
#[clap(name = "Autorenamer", version = "1.0.3", author = "HirschBerge")]

pub struct Autorename {
    #[clap(long = "season", short = 's')]
    season: i32,
    #[clap(long = "path", short = 'p', required = false)]
    path: Option<String>,
}

fn get_files(path: String) -> Result<Vec<String>, Box<dyn Error>> {
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
fn rename_files(files: Result<Vec<String>, Box<dyn Error>>, season: i32, base_path: String) {
    match files {
        Ok(files) => {
            for file in files {
                // let path = Path::new(&file);
                let re = Regex::new(r"Episode [0-9]{1,3}").unwrap();
                if let Some(captures) = re.captures(&file) {
                    if let Some(matched_str) = captures.get(0) {
                        let matched_text = &matched_str.as_str()[8..];
                        let new_name = format!("S{:0>2}E{:0>2}.mp4", season, &matched_text);
                        let new_file_path = format!("{}/{}", &base_path, &new_name); // TODO
                                                                                     // dynamically
                                                                                     // determine
                                                                                     // the
                                                                                     // file
                                                                                     // extension.
                                                                                     // println!("{}", new_file_path);
                        let old_name = format!("{}/{}", base_path, file);
                        println!("{} => {}", file, new_name);
                        let _ = fs::rename(old_name, new_file_path);
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
    let result = get_files(path.clone());
    rename_files(result, season, path);
}
