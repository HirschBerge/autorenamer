use clap::Parser;
use regex::Regex;
use std::{env, error::Error, ffi::OsStr, fs, path::Path};
// use std::fmt::format;

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
                        .contains("Episode ")
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
                let re = Regex::new(r"Episode [0-9]{1,5}").unwrap();
                // let re = Regex::new(r"Episode /d+").unwrap(); // THIS IS SLOW! \d{1,5} is even
                // slower somehow
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

#[cfg(test)]
mod tests {
    use super::get_episodes;
    use std::fs::{create_dir, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_get_episodes_with_matching_files() {
        // Create a temporary directory for testing
        let temp_dir = "test_get_episodes_with_matching_files";
        create_dir(temp_dir).expect("Failed to create temporary directory");

        // Create some files with "Episode" in the name
        create_test_file(&temp_dir, "Episode 1.mp3");
        create_test_file(&temp_dir, "Episode 2.mp3");
        create_test_file(&temp_dir, "Episode 69.mp3");
        create_test_file(&temp_dir, "Not_An_Episode.mp3");

        // Call the function and check the result
        let result = get_episodes(temp_dir.to_string());
        // Clean up: Delete the temporary directory and its contents
        cleanup_temp_directory(temp_dir);
        assert!(result.is_ok());
        let matching_files = result.unwrap();
        assert_eq!(matching_files.len(), 3);
        // assert!(matching_files.contains(&"Episode 1.mp3".to_string()));
        // assert!(matching_files.contains(&"Episode 2.mp3".to_string()));
    }

    #[test]
    fn test_get_episodes_with_no_matching_files() {
        // Create a temporary directory for testing
        let temp_dir = "test_get_episodes_with_no_matching_files";
        create_dir(temp_dir).expect("Failed to create temporary directory");

        // Create some files without "Episode" in the name
        create_test_file(&temp_dir, "Not_An_Episode_1.mp3");
        create_test_file(&temp_dir, "Not_An_Episode_2.mp3");

        // Call the function and check the result
        let result = get_episodes(temp_dir.to_string());
        // Clean up: Delete the temporary directory and its contents
        cleanup_temp_directory(temp_dir);
        assert!(result.is_ok());
        let matching_files = result.unwrap();
        assert!(matching_files.is_empty());
    }

    #[test]
    fn test_get_episodes_with_invalid_directory() {
        // Call the function with a non-existent directory
        let result = get_episodes("non_existent_directory".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Failed to read the directory"
        );
    }

    fn create_test_file(directory: &str, filename: &str) {
        let path = Path::new(directory).join(filename);
        let mut file = File::create(path).expect("Failed to create test file");
        file.write_all(b"Test content")
            .expect("Failed to write to test file");
    }

    fn cleanup_temp_directory(directory: &str) {
        // Delete the temporary directory and its contents
        if Path::new(directory).exists() {
            std::fs::remove_dir_all(directory).expect("Failed to delete temporary directory");
        }
    }
}
