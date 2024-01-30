load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "bazel-lsp",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//src/builtin:builtin_proto_rust",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:either",
        "@crates//:lsp-types",
        "@crates//:starlark",
        "@crates//:starlark_lsp",
        "@crates//:tempfile",
        "@crates//:thiserror",
    ],
    compile_data = ["//src/builtin:builtin.pb"],
)

rust_test(
    name = "unit_tests",
    crate = ":bazel-lsp",
    data = glob(["fixtures/**/*"]),
)
