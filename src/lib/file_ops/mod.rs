use crate::data::SeasonData;
use notify_rust::Notification;
use regex::Regex;
use std::{error::Error, fs};

pub fn get_episodes(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let re = Regex::new(r"(Episode \d{1,5}|E\d{1,5})")?;
    let mut matching_files: Vec<String> = fs::read_dir(path)?
        .flatten() // Flattens the Result<DirEntry, io::Error> into DirEntry by ignoring errors
        .filter_map(|file| {
            let path = file.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    // Check if the file name matches the regex
                    if re.is_match(&file_name_str) {
                        return Some(file_name_str.to_string());
                    }
                }
            }
            None
        })
        .collect();

    matching_files.sort();
    Ok(matching_files)
}

pub fn rename_episodes(
    files: Vec<String>,
    season: i32,
    base_path: String,
    offset: i32,
    dryrun: bool,
    notify: bool,
) {
    let mut printout = true;
    let mut count = 0;
    let workload = files.len();
    if workload >= 100 {
        printout = false;
    }
    files.into_iter().for_each(|file| {
        let current_episode = SeasonData::new(&file, season, &base_path, offset);
        let parsed_data = current_episode.process_episode();
        match parsed_data {
            Ok(data) => {
                let new_name = data.create_new_path(&file, data.create_ext(), false);
                if new_name != data.old_path {
                    if !dryrun {
                        let _ = fs::rename(
                            &data.old_path,
                            data.create_new_path(&file, data.create_ext(), printout),
                        );
                        count += 1;
                    } else {
                        data.create_new_path(&file, data.create_ext(), true);
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    });
    if notify {
        match Notification::new()
            .summary(format!("Renaming Season {} complete", season).as_str())
            .body(format!("A total of {} episodes renamed", count).as_str())
            .appname("anime")
            .timeout(5)
            .show()
        {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        }
    }
}
