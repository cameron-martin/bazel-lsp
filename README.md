# Bazel LSP

This is a LSP for bazel, based on [starlark-rust](https://github.com/facebookexperimental/starlark-rust).

## Features

* Go to definition for identifiers & labels
* Autocomplete for identifiers & labels
* Auto-import (currently only for open files)

## Usage

### Installation

Download a release from github releases and move it to somewhere on your `$PATH`.

Alternatively, it can be installed from source by cloning this repo and building with Bazel:

```sh
bazel build //:bazel-lsp -c opt
```

### VSCode

Ensure you have the [Bazel extension](https://marketplace.visualstudio.com/items?itemName=BazelBuild.vscode-bazel) installed, and add the following config to your user `settings.json`:

```json
{
  "bazel.lsp.command": "bazel-lsp"
}
```

Restart vscode for this to take effect.

### Logging

Logging can be enabled using the `RUST_LOG` environment variable. For more info on the syntax of this environment variable, see the [`tracing_subscriber` docs](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html).

This can be set in vscode by setting the `bazel.lsp.env` configuration option:

```json
{
  "bazel.lsp.env": { "RUST_LOG": "info" },
}
```
