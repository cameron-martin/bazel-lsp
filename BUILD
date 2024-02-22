load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "bazel-lsp",
    srcs = glob(["src/**/*.rs"]),
    compile_data = ["//src/builtin:builtin.pb"],
    deps = [
        "//src/builtin:builtin_proto_rust",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:either",
        "@crates//:hex",
        "@crates//:lsp-types",
        "@crates//:ring",
        "@crates//:serde_json",
        "@crates//:starlark",
        "@crates//:starlark_lsp",
        "@crates//:thiserror",
    ],
)

rust_test(
    name = "unit_tests",
    crate = ":bazel-lsp",
    data = glob(["fixtures/**/*"]),
)
