use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use crate::Error;

/// An iterator to recursively loop through a directory.
/// It yields instances of `Result<PathBuf, recurse::Error>`.
///
/// # Errors
///
/// The yielded items are `Err`'s, if...
/// - ...the initially given path is not directory
/// - ...the initially given path does not exist
/// - ...the user lacks permission for a directory
///
/// The errors contain the underlying `std::io::Error`, which
/// can be inspected to figure out more information about an error.
pub struct Recurse {
    queued_directories: VecDeque<PathBuf>,
    queued_files: VecDeque<PathBuf>,
}

impl Recurse {
    /// Create a new `Recurse` to iterate over `directory`.
    pub fn new<P: AsRef<Path>>(directory: P) -> Self {
        let directory = directory.as_ref().to_path_buf();

        let mut queued_directories = VecDeque::new();
        queued_directories.push_back(directory);

        Self {
            queued_directories,
            queued_files: VecDeque::new(),
        }
    }
}

impl Iterator for Recurse {
    type Item = Result<PathBuf, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.queued_files.is_empty() {
            return Some(Ok(self
                .queued_files
                .pop_front()
                .expect("should not be empty")));
        }

        if !self.queued_directories.is_empty() {
            let directory = self
                .queued_directories
                .pop_front()
                .expect("should not be empty");

            let read_dir = match directory.read_dir() {
                Ok(read_dir) => read_dir,
                Err(e) => {
                    return Some(Err(Error::ReadDir {
                        directory: directory.display().to_string(),
                        e,
                    }))
                }
            };

            for dir_entry in read_dir {
                let path = match dir_entry {
                    Ok(dir_entry) => dir_entry.path(),
                    Err(e) => return Some(Err(Error::DirEntry { e })),
                };

                if path.is_dir() {
                    self.queued_directories.push_back(path);
                } else {
                    self.queued_files.push_back(path);
                }
            }

            return self.next();
        }

        None
    }
}
