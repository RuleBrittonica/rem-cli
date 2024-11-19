    use rem_controller::non_local_controller;
use std::time::Instant;
use log::{
    info,
    error,
};

use crate::error::RefactoringError;

use super::throughput::Controller;

pub fn non_local_controller(
    input: Controller
) -> Result<String, RefactoringError> {
    todo!()
}