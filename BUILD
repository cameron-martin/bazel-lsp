load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "bazel-lsp",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:either",
        "@crates//:lsp-types",
        "@crates//:starlark",
        "@crates//:starlark_lsp",
        "@crates//:tempfile",
        "@crates//:thiserror",
    ],
)

rust_test(
    name = "unit_tests",
    crate = ":bazel-lsp",
    data = glob(["fixtures/**/*"]),
)
