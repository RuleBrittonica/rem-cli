use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ExtractFnBodyError {
    FileReadError(std::io::Error),
    FileWriteError(std::io::Error),
    FileExistsError(std::io::Error),
    FnNotFoundError(String),
    InvalidCursorReference,
}

impl fmt::Display for ExtractFnBodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractFnBodyError::FileReadError(err) => write!(f, "Failed to read file: {}", err),
            ExtractFnBodyError::FileWriteError(err) => write!(f, "Failed to write to file: {}", err),
            ExtractFnBodyError::FnNotFoundError(fn_name) => write!(f, "Function '{}' not found", fn_name),
            ExtractFnBodyError::FileExistsError(file_name) => write!(f, "File to write to '{}' already exists", file_name),
            ExtractFnBodyError::InvalidCursorReference => write!(f, "Invalid cursor reference found in the function body"),
        }
    }
}

impl Error for ExtractFnBodyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ExtractFnBodyError::FileReadError(err) => Some(err),
            ExtractFnBodyError::FileWriteError(err) => Some(err),
            ExtractFnBodyError::FileExistsError(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum TestFailed {
    ControllerFailed(std::io::Error),
    BorrowerFailed(std::io::Error),
    RepairerFailed(std::io::Error),
}

impl fmt::Display for TestFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestFailed::ControllerFailed(err) => write!(f, "Controller test failed: {}", err),
            TestFailed::BorrowerFailed(err) => write!(f, "Borrower test failed: {}", err),
            TestFailed::RepairerFailed(err) => write!(f, "Repairer test failed: {}", err),
        }
    }
}

impl Error for TestFailed {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TestFailed::ControllerFailed(err) => Some(err),
            TestFailed::BorrowerFailed(err) => Some(err),
            TestFailed::RepairerFailed(err) => Some(err),
        }
    }
}