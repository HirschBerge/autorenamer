use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::{env, fs};

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
                        let new_file_path =
                            format!("{}/S{:0>2}E{:0>2}.mp4", &base_path, season, matched_text); // TODO
                                                                                                // dynamically
                                                                                                // determine
                                                                                                // the
                                                                                                // file
                                                                                                // extension.
                                                                                                // println!("{}", new_file_path);
                        let old_name = format!("{}/{}", base_path, file);
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
    let matches = App::new("Rust Autorenamer")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("DIRECTORY")
                .help("Specify the path (optional, defaults to current directory)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("season")
                .short("s")
                .long("season")
                .value_name("SEASON")
                .help("Specify the season as a digit.")
                .required(true),
        )
        .get_matches();
    let path = matches
        .value_of("path")
        .map(|p| p.to_string())
        .unwrap_or_else(|| env::current_dir().unwrap().to_string_lossy().to_string());

    let season = matches
        .value_of("season")
        .unwrap()
        .parse::<i32>()
        .expect("Failed to parse the season as an int.");

    let result = get_files(path.clone());
    rename_files(result, season, path);
}
