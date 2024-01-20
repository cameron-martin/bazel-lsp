mod bazel;
mod client;
mod eval;
mod label;
#[cfg(test)]
pub mod test_fixture;

use std::path::PathBuf;

use bazel::BazelContext;
use clap::Parser;
use client::BazelCli;
use eval::ContextMode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the bazel binary
    #[arg(long, default_value = "bazel")]
    bazel: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let ctx = BazelContext::new(
        BazelCli::new(args.bazel),
        ContextMode::Check,
        true,
        &[],
        true,
    )?;

    starlark_lsp::server::stdio_server(ctx)?;

    Ok(())
}
