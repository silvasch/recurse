fn main() {
    // loop through all entries in the current directory
    for file in recurse::Recurse::new(".") {
        match file {
            Ok(file) => {
                // the 0th component is `.`
                let root = match file.components().nth(1) {
                    Some(root) => root.as_os_str().to_str().unwrap(),
                    None => continue,
                };

                // filter out the `target` and `.git` directories
                if ["target", ".git"].contains(&root) {
                    continue;
                }

                println!("{}", file.display());
            }
            // this error can happen for various reasons:
            // - the initially given path does not exist
            // - the initially given path is not a directory
            // - the user is lacking permission to access a specific file
            //
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}
