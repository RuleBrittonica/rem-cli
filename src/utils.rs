use error::TestFailed;
use std::{
    path::PathBuf,
    io,
    fs,
    error::Error,
};
use git2::Repository;

use crate::tests::{
    controller,
    borrower,
    repairer,
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
        error!("{}" err_msg)
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg)));
    }

    Ok(())
}

/// Runs tests given a path to the test files.
pub fn run_tests(path: std::path::PathBuf) -> Result<(), TestFailed> {
    info!("Running tests from path: {:?}", path);

    if let Err(e) = controller::test() {
        error!("Controller tests failed: {:?}", e);
        return Err(TestFailed::ControllerFailed(e));
    }

    // TODO: Uncomment and fix this once the Borrower tests are stabilized
    // if let Err(e) = borrower::test() {
    //     error!("Borrower tests failed: {:?}", e);
    //     return Err(TestFailed::BorrowerFailed(e));
    // }

    if let Err(e) = repairer::test() {
        error!("Repairer tests failed: {:?}", e);
        return Err(TestFailed::RepairerFailed(e));
    }

    info!("All tests completed successfully");
    Ok(())
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
        eprntln!("{}", err_msg);
        Err(io::Error::new(io::ErrorKind::InvalidInput, err_msg))
    }
}