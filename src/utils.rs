use error::TestFailed;
use std::{
    path::PathBuf,
    io,
    fs,
    error::Error,
    process::exit,
};
use git2::Repository;

use crate::tests::{
    controller,
    borrower,
    repairer,
};

use log::{
    info,
    error,
};


pub enum RepairType {
    Simple,
    LoosestBoundsFirst,
    TightestBoundsFirst,
}

pub fn parse_repair_type ( num: u8 ) -> RepairType {
    match num {
        1 => RepairType::Simple,
        2 => RepairType::LoosestBoundsFirst,
        3 => RepairType::TightestBoundsFirst,
        _ => {
            error!("Invalid Repair Type Specified, program terminating");
            exit(1);
        }
    }
}

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
        error!("{}" err_msg);
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
    let mut total_failed_tests = 0;

    // Run controller tests
    let controller_failed = match controller::test(path) {
        Ok(failed) => failed,
        Err(e) => {
            error!("Controller tests failed: {:?}", e);
            return Err(TestFailed::ControllerFailed(e));
        }
    };
    total_failed_tests += controller_failed;

    // Uncomment and fix this once the Borrower tests are stabilized
    /*
    let borrower_failed = match borrower::test(path) {
        Ok(failed) => failed,
        Err(e) => {
            error!("Borrower tests failed: {:?}", e);
            return Err(TestFailed::BorrowerFailed(e));
        }
    };
    total_failed_tests += borrower_failed;
    */

    // Run repairer tests
    let repairer_failed = match repairer::test(path) {
        Ok(failed) => failed,
        Err(e) => {
            error!("Repairer tests failed: {:?}", e);
            return Err(TestFailed::RepairerFailed(e));
        }
    };
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