load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "bazel-lsp",
    srcs = glob(["src/**/*.rs"]),
    compile_data = [
        "//src/builtin:builtin.pb",
        "//src/builtin:default_build_language.pb",
    ],
    rustc_env_files = [":generate_rustc_env_file"],
    deps = [
        "//src/builtin:build_proto_rust",
        "//src/builtin:builtin_proto_rust",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:prost",
        "@crates//:either",
        "@crates//:hex",
        "@crates//:lsp-types",
        "@crates//:ring",
        "@crates//:serde_json",
        "@crates//:starlark",
        "@crates//:starlark_lsp",
        "@crates//:starlark_syntax",
        "@crates//:thiserror",
    ],
)

genrule(
    name = "generate_rustc_env_file",
    srcs = [
        "Cargo.toml",
        "src/main.rs",
    ],
    outs = ["rustc_env_file"],
    cmd = "echo \"CARGO_PKG_VERSION=$$($(location @rust_host_tools//:cargo) read-manifest | jq -r .version)\" > $@",
    tools = ["@rust_host_tools//:cargo"],
)

rust_test(
    name = "unit_tests",
    crate = ":bazel-lsp",
    data = glob(["fixtures/**/*"]),
)
