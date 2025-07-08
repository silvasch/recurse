/// `recurse`'s error type that can occur when iterating
/// over a directory.
#[derive(Debug)]
pub enum Error {
    /// Something went wrong when listing the files in a directory.
    ReadDir {
        /// The directory where the error occured.
        directory: String,
        /// The underlying error.
        e: std::io::Error,
    },
    /// Something is wrong with a file in a directory.
    DirEntry {
        /// The underlying error.
        e: std::io::Error,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReadDir { directory, e } => {
                write!(f, "reading the directory '{}' failed: {}", directory, e)
            }
            Error::DirEntry { e } => write!(f, "failed to get a dir entry: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReadDir { e, .. } => Some(e),
            Error::DirEntry { e } => Some(e),
        }
    }
}
