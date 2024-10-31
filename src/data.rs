use crate::parsing::parse_episode;
use std::{error::Error, ffi::OsStr, path::Path};
#[derive(Debug)]
pub struct Episode {
    pub old_path: String,
    new_path: String,
}
impl Episode {
    fn new(old_path: String, new_path: String) -> Episode {
        Episode { old_path, new_path }
    }
    pub fn create_ext(&self) -> String {
        Path::new(&self.old_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("mp4")
            .to_string()
    }
    pub fn create_new_path(&self, file: &String, ext: String, print: bool) -> String {
        if print {
            println!(
                "\x1b[31m{}\x1b[0m => \x1b[35m{}.{}\x1b[0m",
                file, &self.new_path, ext
            );
        }
        format!("{}.{}", &self.new_path, ext)
    }
}

#[derive(Debug)]
pub struct SeasonData<'a> {
    pub file: &'a str,
    season: i32,
    base_path: &'a str,
    offset: i32,
}

impl<'a> SeasonData<'a> {
    pub fn new(file: &'a str, season: i32, base_path: &'a str, offset: i32) -> SeasonData<'a> {
        SeasonData {
            file,
            season,
            base_path,
            offset,
        }
    }
    /**
     Process the episode by parsing the episode file, adjusting the episode number based on the offset,
     and creating a new path for the episode file.

     This function takes no parameters and returns a Result containing an Episode on success or a Box<dyn Error> on failure.

     # Errors

     This function can return an error if there is an issue parsing the episode file or creating the new path.

     # Examples

    **/
    pub fn process_episode(&self) -> Result<Episode, Box<dyn Error>> {
        match parse_episode(self.file) {
            Ok((_, (episode_num, title))) => {
                let new_episode_num = episode_num + self.offset;
                let mut new_path = format!(
                    "{}/S{:0>2}E{:0>2}",
                    self.base_path, self.season, new_episode_num
                );
                if !title.is_empty() {
                    new_path = format!("{} {}", new_path, title);
                }
                let old_name = format!("{}/{}", self.base_path, self.file);
                Ok(Episode::new(old_name, new_path))
            }
            Err(err) => Err(Box::<dyn Error>::from(err.to_string())),
        }
    }
}
