mod bazel;
mod eval;

use bazel::BazelContext;
use eval::ContextMode;

fn main() -> anyhow::Result<()> {
    let ctx = BazelContext::new(ContextMode::Check, true, &[], true)?;
    starlark_lsp::server::stdio_server(ctx)?;

    Ok(())
}
