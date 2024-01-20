mod bazel;
mod client;
mod eval;
mod label;
#[cfg(test)]
pub mod test_fixture;

use bazel::BazelContext;
use client::BazelCli;
use eval::ContextMode;

fn main() -> anyhow::Result<()> {
    let ctx = BazelContext::new(BazelCli, ContextMode::Check, true, &[], true)?;
    starlark_lsp::server::stdio_server(ctx)?;

    Ok(())
}
