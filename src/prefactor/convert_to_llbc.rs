//! The goal of this module is to allow the user to convert the project from a
//! Rust project to a .llbc file. This is done by using the CHARON tool.
//! The CHARON tool is located in the `tools/charon` directory, as outlined by
//! the Config.toml file.
//!
//! This process is run both before the refactor process, to generate a .llbc of
//! the original project, and after the refactor process, to generate a .llbc of
//! the refactored project.
//! At some point it may need to be modified to allow for incremental
//! compilation. But at this stage, the goal is to get the basic functionality
//! working first.