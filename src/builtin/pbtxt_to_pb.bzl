def _pbtxt_to_pb_impl(ctx):
    descriptors = ctx.attr.message_proto_library[ProtoInfo].transitive_descriptor_sets
    joined_descriptors = ctx.configuration.host_path_separator.join([d.path for d in descriptors.to_list()])

    command = "cat {pbtxt} | {protoc} --encode={message_type} --deterministic_output --descriptor_set_in='{descriptors}' > {pb}".format(
        pbtxt = ctx.file.pbtxt.path,
        protoc = ctx.executable._protoc.path,
        message_type = ctx.attr.message_type,
        descriptors = joined_descriptors,
        pb = ctx.outputs.pb.path,
    )

    ctx.actions.run_shell(
        outputs = [ctx.outputs.pb],
        inputs = depset(transitive = [ctx.attr.pbtxt.files, descriptors]),
        mnemonic = "PbtxtToPb",
        command = command,
        tools = [ctx.executable._protoc],
    )

    return [DefaultInfo(files = depset(direct = [ctx.outputs.pb]))]

pbtxt_to_pb = rule(
    implementation = _pbtxt_to_pb_impl,
    attrs = {
        "pbtxt": attr.label(allow_single_file = True, mandatory = True),
        "pb": attr.output(mandatory = True),
        "message_type": attr.string(mandatory = True),
        "message_proto_library": attr.label(mandatory = True, providers = [ProtoInfo]),
        "_protoc": attr.label(default = "@protobuf//:protoc", executable = True, cfg = "exec"),
    },
    doc = "Converts proto message of type `message type` in `pbtxt` to binary format in `pb`. " +
          "The `message_proto_library` points to `proto_library` for the given `message_type`",
)
