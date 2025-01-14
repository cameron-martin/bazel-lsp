load("@rules_rust_prost//:defs.bzl", "rust_prost_library")
load(":pbtxt_to_pb.bzl", "pbtxt_to_pb")

proto_library(
    name = "builtin_proto",
    srcs = [
        "builtin.proto",
    ],
)

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

# The pbtxt file can be obtained from bazel instance:
# bazel info build-language | protoc --decode=blaze_query.BuildLanguage --proto_path <BAZEL_REPO> src/main/protobuf/build.proto > default_build_language.pbtxt
pbtxt_to_pb(
    name = "default_build_language",
    message_proto_library = "@bazel_tools//src/main/protobuf:build_proto",
    message_type = "blaze_query.BuildLanguage",
    pb = "default_build_language.pb",
    pbtxt = "default_build_language.pbtxt",
    visibility = ["//visibility:public"],
)

# The pbtxt file can be obtained from within bazel repository:
#
# bazel build src/main/java/com/google/devtools/build/lib:gen_api_proto
# cat bazel-bin/src/main/java/com/google/devtools/build/lib/builtin.pb | protoc --decode=builtin.Builtins --proto_path src/main/protobuf builtin.proto > builtin.pbtxt
pbtxt_to_pb(
    name = "builtin",
    message_proto_library = ":builtin_proto",
    message_type = "builtin.Builtins",
    pb = "builtin.pb",
    pbtxt = "builtin.pbtxt",
    visibility = ["//visibility:public"],
)
