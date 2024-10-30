use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, ffi::OsStr, path::Path};

#[derive(Debug)]
pub(crate) struct Episode {
    pub(crate) old_path: String,
    new_path: String,
}
impl Episode {
    fn new(old_path: String, new_path: String) -> Episode {
        Episode { old_path, new_path }
    }
    pub(crate) fn create_ext(&self) -> String {
        Path::new(&self.old_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("mp4")
            .to_string()
    }
    pub(crate) fn create_new_path(&self, base_path: String, ext: String, file: String) -> String {
        println!(
            "\x1b[31m{}\x1b[0m => \x1b[35m{}.{}\x1b[0m",
            file, &self.new_path, ext
        );
        format!("{}/{}.{}", base_path, &self.new_path, ext)
    }
}

lazy_static! {
    static ref EPISODE_REGEX: Regex = Regex::new(r"(Episode [0-9]{1,5})(.*?)(\.)").unwrap();
}

#[derive(Debug)]
pub struct SeasonData<'a> {
    pub(crate) file: &'a str,
    season: i32,
    base_path: &'a str,
    offset: i32,
}

impl<'a> SeasonData<'a> {
    pub(crate) fn new(
        file: &'a str,
        season: i32,
        base_path: &'a str,
        offset: i32,
    ) -> SeasonData<'a> {
        SeasonData {
            file,
            season,
            base_path,
            offset,
        }
    }
    pub fn process_episode(&self) -> Result<Episode, Box<dyn Error>> {
        // Check for matches in the file name
        if let Some(captures) = EPISODE_REGEX.captures(self.file) {
            // Extract episode number
            if let Some(matched_str) = captures.get(1) {
                let episode_str = &matched_str.as_str()[8..]; // "Episode " is 8 chars long
                if let Ok(episode_num) = episode_str.parse::<i32>() {
                    // Adjust episode number by the offset
                    let new_episode_num = episode_num + self.offset;
                    let mut new_path = format!("S{:0>2}E{:0>2}", self.season, new_episode_num);
                    let description = captures.get(2).map(|m| m.as_str().trim()).unwrap_or("");
                    if !description.is_empty() {
                        new_path = format!("{} {}", new_path, description);
                    }
                    let old_name = format!("{}/{}", self.base_path, self.file);
                    Ok(Episode::new(old_name, new_path))
                } else {
                    Err(format!("Failed to parse episode number in '{}'", self.file).into())
                }
            } else {
                Err(format!("Pattern not found in '{}'", self.file).into())
            }
        } else if Path::new(&self.file).extension().is_none() {
            Err(format!("File '{}' has no extension, skipping!", self.file).into())
        } else {
            Err(format!("Pattern not found in '{}'", self.file).into())
        }
    }
}
