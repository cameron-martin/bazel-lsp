# Bazel LSP

This is a LSP for bazel, forked from [starlark-rust](https://github.com/facebookexperimental/starlark-rust).

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

TODO