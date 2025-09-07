use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub operations: PathBuf,
}
