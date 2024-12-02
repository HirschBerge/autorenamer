#[cfg(test)]
mod tests {
    use autorenamer::file_ops::{get_episodes, rename_episodes};
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
        rename_episodes(files, season, base_path.clone(), offset, false, false);

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
        rename_episodes(files, season, base_path.clone(), offset, false, false);
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
        create_test_file(temp_dir, "S22E73 A Massive Change.mp3");
        create_test_file(temp_dir, "Not_An_Episode.mp3");
        // Call the function and check the result
        let result = get_episodes(temp_dir.to_string());
        // Clean up: Delete the temporary directory and its contents
        cleanup_temp_directory(temp_dir);
        assert!(result.is_ok());
        let matching_files = result.unwrap();
        assert_eq!(matching_files.len(), 4);
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
        create_test_file(temp_dir, "S01E03 This a name also.mp3");
        create_test_file(temp_dir, "Episode 69 NIIIICE.mp3");
        create_test_file(temp_dir, "Not_An_Episode.mp3");
        // Call the function and check the result
        let result = get_episodes(temp_dir.to_string());
        match result {
            Ok(result) => {
                rename_episodes(result, 1, temp_dir.to_string(), 0, false, false);
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
        assert!(matching_files.contains(&"S01E03 This a name also.mp3".to_string()));
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
