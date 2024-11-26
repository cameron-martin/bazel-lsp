mod bazel;
mod builtin;
mod client;
mod file_type;
mod label;
#[cfg(test)]
pub mod test_fixture;
mod workspace;

use std::{env, path::PathBuf};

use bazel::BazelContext;
use clap::Parser;
use client::BazelCli;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the bazel binary
    #[arg(long, default_value = "bazel")]
    bazel: PathBuf,

    /// Whether to use a separate output base for bazel queries.
    ///
    /// This makes concurrent builds not block queries.
    #[arg(long)]
    no_distinct_output_base: bool,

    /// The directory to put the query output bases in.
    ///
    /// This is ignored if `--no-distinct-output-base` is enabled.
    /// By default this is the directory `bazel-lsp` in the OS's
    /// temp directory.
    #[arg(long)]
    query_output_base: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let query_output_base = if args.no_distinct_output_base {
        None
    } else {
        Some(
            args.query_output_base
                .unwrap_or_else(|| env::temp_dir().join("bazel-lsp")),
        )
    };

    let ctx = BazelContext::new(
        BazelCli::new(args.bazel),
        query_output_base,
    )?;

    starlark_lsp::server::stdio_server(ctx)?;

    Ok(())
}
