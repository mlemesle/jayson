use clap::Parser;

use crate::{args::Args, operations::Operations};

mod args;
mod operations;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let operations = Operations::try_new(&args.operations)?;

    let json = serde_json::json!({
        "hello": "there !"
    });

    let modified = operations.process(&json)?;

    println!("{}", serde_json::to_string_pretty(&modified)?);

    Ok(())
}
