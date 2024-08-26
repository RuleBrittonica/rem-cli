


/// Removes the file extension from the given file path and returns the resulting path as a `String`.
/// If the path does not have an extension, the original path is returned.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path from which the extension should be stripped.
///
/// # Returns
///
/// * `String` - The path without the file extension.
///
/// # Example
///
/// ```
/// let path = "example.rs";
/// let stripped = strip_extension(path);
/// assert_eq!(stripped, "example");
/// ```
pub fn strip_extension(path: &str) -> String {
    // Create a Path object from the provided string slice.
    let path = std::path::Path::new(path);

    // Get the file stem (the file name without the extension) as an OsStr.
    let stem = path.file_stem().unwrap_or_else(|| path.as_os_str());

    // Convert the OsStr to a String and return it.
    stem.to_string_lossy().to_string()
}

pub enum ProgramOptions {
    Controller,
    Borrower,
    Repairer,
    All,
}
