pub use build_proto::blaze_query::*;
pub use builtin_proto::builtin::*;
use starlark::{
    docs::{DocFunction, DocMember, DocParam, DocProperty, DocString},
    typing::Ty,
};

use crate::file_type::FileType;

/// Names of globals missing in builtins reported by bazel.
/// See e.g. https://github.com/bazel-contrib/vscode-bazel/issues/1#issuecomment-2036369868
pub static MISSING_GLOBALS: &'static[&'static str] = &[
    // All values from https://bazel.build/rules/lib/globals/workspace
    "bind",
    "register_execution_platforms",
    "register_toolchains",
    "workspace",

    // Values from https://bazel.build/rules/lib/globals/module
    "archive_override",
    "bazel_dep",
    "git_override",
    "include",
    "inject_repo",
    "local_path_override",
    "module",
    "multiple_version_override",
    "override_repo",
    "register_execution_platforms",
    "register_toolchains",
    "single_version_override",
    "use_extension",
    "use_repo",
    "use_repo_rule",

    // Missing values from https://bazel.build/rules/lib/globals/build
    "package",
    "repo_name",

    // Missing values from https://bazel.build/rules/lib/globals/bzl
    "exec_transition",
    "module_extension",
    "repository_rule",
    "tag_class",

    // Marked as not documented on https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/packages/BuildGlobals.java
    "licenses",
    "environment_group",
    // Removed in https://github.com/bazelbuild/bazel/commit/5ade9da5de25bc93d0ec79faea8f08a54e5b9a68
    "distribs",
];

pub fn build_language_to_doc_members<'a>(
    build_language: &'a BuildLanguage,
) -> impl Iterator<Item = (String, DocMember)> + 'a {
    build_language
        .rule
        .iter()
        .map(|rule| (rule.name.clone(), rule_to_doc_member(rule)))
}

pub fn rule_to_doc_member(rule: &RuleDefinition) -> DocMember {
    DocMember::Function(DocFunction {
        docs: rule
            .documentation
            .as_ref()
            .and_then(|doc| create_docstring(doc)),
        params: rule
            .attribute
            .iter()
            .map(|attribute| DocParam::Arg {
                name: attribute.name.clone(),
                docs: attribute
                    .documentation
                    .as_ref()
                    .and_then(|doc| create_docstring(doc)),
                typ: Ty::any(),
                default_value: None,
            })
            .collect(),
        ..Default::default()
    })
}

pub fn builtins_to_doc_members<'a>(
    builtins: &'a Builtins,
    file_type: FileType,
) -> impl Iterator<Item = (String, DocMember)> + 'a {
    builtins.global.iter().flat_map(move |global| {
        if global.api_context == ApiContext::All as i32
            || (global.api_context == ApiContext::Bzl as i32 && file_type == FileType::Library)
            || (global.api_context == ApiContext::Build as i32 && file_type == FileType::Build)
        {
            Some((global.name.clone(), value_to_doc_member(global)))
        } else {
            None
        }
    })
}

fn value_to_doc_member(value: &Value) -> DocMember {
    let docs = create_docstring(&value.doc);

    if let Some(callable) = &value.callable {
        DocMember::Function(DocFunction {
            docs,
            params: callable
                .param
                .iter()
                .map(|param| DocParam::Arg {
                    name: param.name.clone(),
                    docs: create_docstring(&param.doc),
                    typ: Ty::any(),
                    default_value: if param.is_mandatory {
                        None
                    } else {
                        Some(param.default_value.clone())
                    },
                })
                .collect(),
            ..Default::default()
        })
    } else {
        DocMember::Property(DocProperty {
            docs,
            typ: Ty::any(),
        })
    }
}

fn create_docstring(summary: &str) -> Option<DocString> {
    let summary = summary.trim();

    if summary.is_empty() {
        None
    } else {
        Some(DocString {
            summary: summary.to_string(),
            details: None,
        })
    }
}
