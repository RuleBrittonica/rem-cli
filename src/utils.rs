use std::{
    path::PathBuf,
    io,
    fs,
    error::Error,
    process::exit,
    time::SystemTime,
};
use git2::Repository;

use log::{
    info,
    error,
};

use crate::tests::{
    controller,
    borrower,
    repairer,
};

use crate::error::TestFailed;

/// Downloads a repo from a URL.
/// # Returns
/// * The path to the git-repo it has cloned.
pub fn get_from_git(repo_url: String) -> Result<PathBuf, Box<dyn Error>> {
    // Define a path where the repo will be cloned
    let path: PathBuf = PathBuf::from("tmp_test_src");

    // Clone the repository
    Repository::clone(&repo_url, &path)?;

    Ok(path)
}

/// Deletes a cloned repo from the user's HDD.
pub fn delete_repo(path: PathBuf) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        fs::remove_dir_all(&path)?;
    } else {
        // Return an error if the path is not a directory
        let err_msg = format!("The specified path is not a directory: {:?}", path);
        error!("{}", err_msg);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg)));
    }

    Ok(())
}

/// Runs tests given a path to the test files.
/// # Returns
/// * `u8` The number of tests that failed. Returning 0 is ideal!
pub fn run_tests(path: std::path::PathBuf) -> Result<u8, TestFailed> {
    info!("Running tests from path: {:?}", path);

    // Initialize the total number of failed tests
    let mut total_failed_tests: u8 = 0;

    // Run controller tests
    let controller_failed: u8 = match controller::test(path.clone()) {
        Ok(failed) => failed,
        Err(e) => {
            error!("Controller tests failed: {:?}", e);
            return Err(TestFailed::ControllerFailed(e));
        }
    };
    info!("Controller tests successful, {} tests failed", controller_failed);
    total_failed_tests += controller_failed;

    let borrower_failed: u8 = match borrower::test(path.clone()) {
        Ok(failed) => failed,
        Err(e) => {
            error!("Borrower tests failed: {:?}", e);
            return Err(TestFailed::BorrowerFailed(e));
        }
    };
    info!("Borrower tests successful, {} tests failed", borrower_failed);
    total_failed_tests += borrower_failed;

    // Run repairer tests
    let repairer_failed: u8 = match repairer::test(path.clone()) {
        Ok(failed) => failed,
        Err(e) => {
            error!("Repairer tests failed: {:?}", e);
            return Err(TestFailed::RepairerFailed(e));
        }
    };
    info!("Repairer tests succesful, {} tests failed", repairer_failed);
    total_failed_tests += repairer_failed;

    info!("All tests completed. Total failed tests: {}", total_failed_tests);
    Ok(total_failed_tests)
}

/// Deletes a backup file or directory at the given path.
pub fn delete_backup(backup_path: PathBuf) -> Result<(), io::Error> {
    if backup_path.is_file() {
        // Delete the file if it's a file
        match fs::remove_file(&backup_path) {
            Ok(_) => {
                info!("Backup file deleted successfully: {:?}", backup_path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete backup file: {:?}", e);
                Err(e)
            }
        }
    } else if backup_path.is_dir() {
        // Delete the directory and its contents if it's a directory
        match fs::remove_dir_all(&backup_path) {
            Ok(_) => {
                info!("Backup directory deleted successfully: {:?}", backup_path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete backup directory: {:?}", e);
                Err(e)
            }
        }
    } else {
        // If the path is neither a file nor a directory, return an error
        let err_msg = format!("The specified backup path is neither a file nor a directory: {:?}", backup_path);
        error!("{}", err_msg); // Output to logs and std err.
        eprintln!("{}", err_msg);
        Err(io::Error::new(io::ErrorKind::InvalidInput, err_msg))
    }
}

pub fn handle_result(success: bool, run: &str, msg: &str) {
    if success {
        info!("Program {} was successful.", run);
    } else {
        error!("Program {} failed: {}", run, msg);
        exit(1);
    }
}

pub fn backup_file(original_path: PathBuf) -> Option<PathBuf> {
    // Check if the path is a file; directories are not supported
    if !original_path.is_file() {
        let err_msg = "Path must be a file";
        error!("{}: {:?}", err_msg, original_path);
        panic!("{}", err_msg);
    }

    // Get the current timestamp
    let now = SystemTime::now();
    let timestamp = match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => {
            let err_msg = "Failed to get timestamp";
            error!("{}", err_msg);
            panic!("{}", err_msg);
        }
    };

    // Construct the backup path
    let parent_dir = match original_path.parent() {
        Some(parent) => parent,
        None => {
            let err_msg = "No parent directory found";
            error!("{}", err_msg);
            panic!("{}", err_msg);
        }
    };

    // let file_name = match original_path.file_name() {
    //     Some(name) => name.to_string_lossy(),
    //     None => {
    //         let err_msg = "No file name found";
    //         error!("{}", err_msg);
    //         panic!("{}", err_msg);
    //     }
    // };

    let file_stem = match original_path.file_stem() {
        Some(stem) => stem.to_string_lossy(),
        None => {
            let err_msg = "No file stem found";
            error!("{}", err_msg);
            panic!("{}", err_msg);
        }
    };

    let extension = match original_path.extension() {
        Some(ext) => ext.to_string_lossy(),
        None => String::new().into(), // No extension
    };

    let backup_file_name = format!("{}_backup_{}.{}", file_stem, timestamp, extension);
    let backup_path = parent_dir.join(backup_file_name);

    // Attempt to copy the file
    if let Err(e) = fs::copy(&original_path, &backup_path) {
        error!("Failed to copy file from {:?} to {:?}: {}", original_path, backup_path, e);
        panic!("Failed to copy file from {:?} to {:?}: {}", original_path, backup_path, e);
    }

    info!("Successfully backed up file from {:?} to {:?}", original_path, backup_path);
    Some(backup_path)
}