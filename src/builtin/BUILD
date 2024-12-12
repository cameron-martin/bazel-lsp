load("@rules_rust_prost//:defs.bzl", "rust_prost_library")

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
genrule(
    name = "default_build_language_pbtxt_to_pb",
    srcs = [
        ":default_build_language.pbtxt",
    ],
    outs = ["default_build_language.pb"],
    tools = [
        "@protobuf//:protoc",
        "@bazel_tools//src/main/protobuf:build_proto",
    ],
    cmd = "cat $(execpath :default_build_language.pbtxt) | " +
        "$(execpath @protobuf//:protoc) " +
        "--encode=blaze_query.BuildLanguage " +
        "--deterministic_output " +
        "--descriptor_set_in=$(execpath @bazel_tools//src/main/protobuf:build_proto) " +
        "> $@",
    visibility = ["//visibility:public"],
)

# The pbtxt file can be obtained from within bazel repository:
#
# bazel build src/main/java/com/google/devtools/build/lib:gen_api_proto
# cat bazel-bin/src/main/java/com/google/devtools/build/lib/builtin.pb | protoc --decode=builtin.Builtins --proto_path src/main/protobuf builtin.proto > builtin.pbtxt
genrule(
    name = "builtin_pbtxt_to_pb",
    srcs = [
        ":builtin.pbtxt",
    ],
    outs = ["builtin.pb"],
    tools = [
        "@protobuf//:protoc",
        "@bazel_tools//src/main/protobuf:builtin_proto",
    ],
    cmd = "cat $(execpath :builtin.pbtxt) | " +
        "$(execpath @protobuf//:protoc) " +
        "--encode=builtin.Builtins " +
        "--deterministic_output " +
        "--descriptor_set_in=$(execpath @bazel_tools//src/main/protobuf:builtin_proto) " +
        "> $@",
    visibility = ["//visibility:public"],
)
