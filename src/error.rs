use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum UpdateError {
    ExtractNoWorkingCode,
    ExtractNoCallerFnName,
    ControllerNoWorkingCode,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateError::ExtractNoWorkingCode => write!(f, "No working code found in the extract function"),
            UpdateError::ExtractNoCallerFnName => write!(f, "No caller function name found in the extract function"),
            UpdateError::ControllerNoWorkingCode => write!(f, "No working code found in the controller function"),
        }
    }
}

#[derive(Debug)]
pub enum RefactoringError {

}

#[derive(Debug)]
pub enum TestFailed {
    ExtractFailed(std::io::Error),
    ControllerFailed(std::io::Error),
    BorrowerFailed(std::io::Error),
    RepairerFailed(std::io::Error),
}

impl fmt::Display for TestFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestFailed::ExtractFailed(err) => write!(f, "Extract test failed: {}", err),
            TestFailed::ControllerFailed(err) => write!(f, "Controller test failed: {}", err),
            TestFailed::BorrowerFailed(err) => write!(f, "Borrower test failed: {}", err),
            TestFailed::RepairerFailed(err) => write!(f, "Repairer test failed: {}", err),
        }
    }
}

impl Error for TestFailed {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TestFailed::ExtractFailed(err) => Some(err),
            TestFailed::ControllerFailed(err) => Some(err),
            TestFailed::BorrowerFailed(err) => Some(err),
            TestFailed::RepairerFailed(err) => Some(err),
        }
    }
}