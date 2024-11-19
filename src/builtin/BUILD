load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")

rust_prost_library(
    name = "build_proto_rust",
    proto = "@bazel_tools//src/main/protobuf:build_proto",
    visibility = ["//visibility:public"],
)

rust_prost_library(
    name = "builtin_proto_rust",
    proto = "@bazel_tools//src/main/protobuf:builtin_proto",
    visibility = ["//visibility:public"],
)

exports_files([
    # Built from the bazel repo using:
    # `bazel build //src/main/java/com/google/devtools/build/lib:gen_api_proto`
    "builtin.pb",
    # Obtained from bazel instance with:
    # `bazel info build-language`
    "default_build_language.pb",
])
