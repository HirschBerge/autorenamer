use clap::Parser;
use regex::Regex;
use std::{env, error::Error, ffi::OsStr, fs, path::Path};
// use std::fmt::format;

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
        Path::new(&self.old_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("mp4")
            .to_string()
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
    let matching_files: Vec<String> = fs::read_dir(path)?
        .flatten() // Flattens the Result<DirEntry, io::Error> into DirEntry by ignoring errors
        .filter_map(|file| {
            let path = file.path();
            if path.is_file() {
                // Check for "Episode " in the file name
                // TODO: Include files with "^E[0-9]{2,3}" so that already-processed files can be included.
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.contains("Episode ") {
                        return Some(file_name_str.to_string()); // Push the matching file name
                    }
                }
            }
            None
        })
        .collect();

    Ok(matching_files)
}

fn rename_episodes(
    files: Result<Vec<String>, Box<dyn Error>>,
    season: i32,
    base_path: String,
    offset: i32,
    dryrun: bool,
) {
    match files {
        Ok(files) => {
            for file in files {
                // TODO: Include files with "^E[0-9]{2,3}" so that already-processed files can be included.
                let re = Regex::new(r"Episode [0-9]{1,5}").unwrap(); // NOTE: This is probably fine to leave. It works and doesn't overflow the size. See the help. this is a comppile-time error
                if let Some(captures) = re.captures(&file) {
                    if let Some(matched_str) = captures.get(0) {
                        // Extract the numeric part (episode number) from the matched string
                        let episode_str = &matched_str.as_str()[8..]; // "Episode " is 8 chars long
                                                                      // Convert the episode number string to i32
                        if let Ok(episode_num) = episode_str.parse::<i32>() {
                            // Add the offset to the episode number
                            let new_episode_num = episode_num + offset; // NOTE: offset defaults to 0 so does nothing if an offset is not provided.
                            let new_name = format!("S{:0>2}E{:0>2}", season, new_episode_num);
                            // Get old and new file paths
                            let old_name = format!("{}/{}", base_path, file);
                            let episode = Episode::new(old_name, new_name);
                            // Perform the file renaming
                            if !dryrun {
                                let _ = fs::rename(
                                    &episode.old_path,
                                    episode.create_new_path(
                                        base_path.clone(),
                                        episode.create_ext(),
                                        file,
                                    ),
                                );
                            } else {
                                episode.create_new_path(
                                    base_path.clone(),
                                    episode.create_ext(),
                                    file,
                                );
                            }
                        } else {
                            println!("Failed to parse episode number.");
                        }
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
    println!("Ran with parameter dryrun set to '{}'.\nIf true, changes are only printed to screen and not reflected in reality.", &dryrun);
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
    rename_episodes(result, season, path, offset, dryrun);
}

#[cfg(test)]
mod tests {
    use super::{get_episodes, rename_episodes};
    use std::error::Error;
    use std::fs::{create_dir, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_addition() {
        // Setup test environment
        let test_dir = "test_addition_dir";
        create_dir(test_dir).expect("Failed to create test directory");

        // Create a test file that matches the episode pattern
        let test_file = "Episode 05.mp4";
        create_test_file(test_dir, test_file);

        // Prepare the input for the rename_episodes function
        let files: Result<Vec<String>, Box<dyn Error>> = Ok(vec![test_file.to_string()]);
        let season = 1;
        let base_path = test_dir.to_string();
        let offset = 1;

        // Run the rename_episodes function
        rename_episodes(files, season, base_path.clone(), offset, false);

        // Assert that the file was renamed correctly
        let expected_new_name = format!("{}/S01E06.mp4", test_dir); // Episode number is 05 + offset (1) = 06
        assert!(
            Path::new(&expected_new_name).exists(),
            "Renamed file not found"
        );

        // Clean up the test environment
        cleanup_temp_directory(test_dir);
    }
    #[test]
    fn test_subtraction() {
        // Setup test environment
        let test_dir = "test_subtraction_dir";
        create_dir(test_dir).expect("Failed to create test directory");
        // Create a test file that matches the episode pattern
        let test_file = "Episode 05.mp4";
        create_test_file(test_dir, test_file);
        // Prepare the input for the rename_episodes function
        let files: Result<Vec<String>, Box<dyn Error>> = Ok(vec![test_file.to_string()]);
        let season = 1;
        let base_path = test_dir.to_string();
        let offset = -1;
        // Run the rename_episodes function
        rename_episodes(files, season, base_path.clone(), offset, false);
        // Assert that the file was renamed correctly
        let expected_new_name = format!("{}/S01E04.mp4", test_dir); // Episode number is 05 + offset (1) = 06
        assert!(
            Path::new(&expected_new_name).exists(),
            "Renamed file not found"
        );
        // Clean up the test environment
        cleanup_temp_directory(test_dir);
    }
    #[test]
    fn test_get_episodes_with_matching_files() {
        // Create a temporary directory for testing
        let temp_dir = "test_get_episodes_with_matching_files";
        create_dir(temp_dir).expect("Failed to create temporary directory");

        // Create some files with "Episode" in the name
        create_test_file(temp_dir, "Episode 1.mp3");
        create_test_file(temp_dir, "Episode 2.mp3");
        create_test_file(temp_dir, "Episode 69.mp3");
        create_test_file(temp_dir, "Not_An_Episode.mp3");
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
        create_test_file(temp_dir, "Not_An_Episode_1.mp3");
        create_test_file(temp_dir, "Not_An_Episode_2.mp3");

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
            "No such file or directory (os error 2)"
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
