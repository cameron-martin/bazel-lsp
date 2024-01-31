# Copied from: https://github.com/bazelbuild/bazel/blob/master/src/main/protobuf/BUILD

load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")

proto_library(
    name = "builtin_proto",
    srcs = ["builtin.proto"],
)

rust_prost_library(
    name = "builtin_proto_rust",
    proto = ":builtin_proto",
    visibility = ["//visibility:public"],
)

# Built from the bazel repo using:
# `bazel build //src/main/java/com/google/devtools/build/lib:gen_api_proto`
exports_files(["builtin.pb"])
