use std::time::Instant;
use log::{
    info,
    error,
};
use rem_repairer::common::{
    RepairSystem,
    RepairResult,
};
use crate::error::RefactoringError;

use super::throughput::Repairer;

pub fn repair_lifetime(
    input: Repairer,
) -> Result<String, RefactoringError> {

    todo!()

}

pub fn repair_lifetime_rustc(
    input: Repairer,
) -> Result<String, RefactoringError> {
    todo!()
}

pub fn repair_lifetime_cargo(
    input: Repairer,
) -> Result<String, RefactoringError> {
    todo!()
}