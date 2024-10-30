use clap::Parser;
use std::{env, error::Error, fs};
mod data;
use crate::data::SeasonData;

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

fn get_episodes(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let tacos: i32 = 0;
    let mut matching_files: Vec<String> = fs::read_dir(path)?
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

    matching_files.sort();
    Ok(matching_files)
}

fn rename_episodes(files: Vec<String>, season: i32, base_path: String, offset: i32, dryrun: bool) {
    for file in files {
        let current_episode = SeasonData::new(&file, season, &base_path, offset);
        let parsed_data = current_episode.process_episode();
        match parsed_data {
            Ok(data) => {
                if !dryrun {
                    let _ = fs::rename(
                        &data.old_path,
                        data.create_new_path(
                            base_path.clone(),
                            data.create_ext(),
                            current_episode.file.to_string(),
                        ),
                    );
                } else {
                    data.create_new_path(base_path.clone(), data.create_ext(), file);
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
    println!(
        "Ran with parameter dryrun set to '{}'.\nIf true, changes are only printed to screen and not reflected in reality.",
        dryrun
    );
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
            rename_episodes(result, season, path, offset, dryrun);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_episodes, rename_episodes};
    use std::fs::{create_dir, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_addition() {
        // Setup test environment
        let test_dir = "test_addition_dir";
        // Clean up the test environment
        cleanup_temp_directory(test_dir);
        create_dir(test_dir).expect("Failed to create test directory");

        // Create a test file that matches the episode pattern
        let test_file = "Episode 05.mp4";
        create_test_file(test_dir, test_file);

        // Prepare the input for the rename_episodes function
        let files = vec![test_file.to_string()];
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
        // Clean up the test environment
        cleanup_temp_directory(test_dir);
        create_dir(test_dir).expect("Failed to create test directory");
        // Create a test file that matches the episode pattern
        let test_file = "Episode 05.mp4";
        create_test_file(test_dir, test_file);
        // Prepare the input for the rename_episodes function
        let files = vec![test_file.to_string()];
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
    fn test_episodes_with_names() {
        // Create a temporary directory for testing
        let temp_dir = "test_episodes_with_names";
        // Clean up the test environment
        cleanup_temp_directory(temp_dir);
        create_dir(temp_dir).expect("Failed to create temporary directory");

        // Create some files with "Episode" in the name
        create_test_file(temp_dir, "Episode 1 Has a name.mp3");
        create_test_file(temp_dir, "Episode 2 Has a name also.mp3");
        create_test_file(temp_dir, "Episode 69 NIIIICE.mp3");
        create_test_file(temp_dir, "Not_An_Episode.mp3");
        // Call the function and check the result
        let result = get_episodes(temp_dir.to_string());
        match result {
            Ok(result) => {
                rename_episodes(result, 1, temp_dir.to_string(), 0, false);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        // Clean up: Delete the temporary directory and its contents
        let matching_files = get_filenames_in_directory(temp_dir).unwrap();
        cleanup_temp_directory(temp_dir);

        assert!(matching_files.contains(&"S01E01 Has a name.mp3".to_string()));
        assert!(matching_files.contains(&"S01E02 Has a name also.mp3".to_string()));
        assert!(matching_files.contains(&"S01E69 NIIIICE.mp3".to_string()));
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
            result.err().unwrap().to_string(),"No such file or directory (os error 3)"
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
    fn get_filenames_in_directory(dir_path: &str) -> std::io::Result<Vec<String>> {
        let mut filenames = Vec::new();

        // Read the directory
        for entry in std::fs::read_dir(Path::new(dir_path))? {
            let entry = entry?;
            // Get the file name and convert it to a String
            let file_name = entry.file_name().into_string().unwrap_or_default();
            filenames.push(file_name);
        }

        Ok(filenames)
    }
}
