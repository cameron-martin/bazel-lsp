# Development

## VS Code setup

Use the [rules_rust rust-analyzer integration](https://bazelbuild.github.io/rules_rust/rust_analyzer.html)
to configure Rust language support with the project's Bazel toolchain.

1. Open the repository root in VS Code.
2. Install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
3. Run the setup command from the repository root:

   ```sh
   bazel run @rules_rust//tools/rust_analyzer:setup -- vscode
   ```

4. Run **Developer: Reload Window** from the VS Code command palette.

Re-run setup after changing the Rust toolchain or `MODULE.bazel`, or after
`bazel clean --expunge`.

If symbols or dependencies appear incorrect, restart rust-analyzer. If the
problem persists, clear its discovery cache and regenerate the setup:

```sh
bazel run @rules_rust//tools/rust_analyzer:setup -- --clean vscode
```
