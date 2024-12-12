use std::sync::LazyLock;

pub use build_proto::blaze_query::*;
pub use builtin_proto::builtin::*;
use htmd::{Element, HtmlToMarkdown};
use starlark::{
    docs::{DocFunction, DocMember, DocParam, DocParams, DocProperty, DocString},
    typing::Ty,
};

use crate::file_type::FileType;

/// Names of globals missing in builtins reported by bazel.
/// See e.g. https://github.com/bazel-contrib/vscode-bazel/issues/1#issuecomment-2036369868
pub static MISSING_GLOBALS: &'static [&'static str] = &[
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

static HTML_CONVERTER: LazyLock<htmd::HtmlToMarkdown> = LazyLock::new(|| {
    HtmlToMarkdown::builder()
        .add_handler(vec!["pre"], |element: Element| {
            for attr in element.attrs {
                if &attr.name.local == "class" && attr.value.to_string() == "language-python" {
                    return Some(format!("\n```python\n{}\n```\n", element.content));
                }
            }
            Some(element.content.to_string())
        })
        .add_handler(vec!["a"], |element: Element| {
            for attr in element.attrs {
                if &attr.name.local == "href" {
                    // For local links, just remove link altogether.
                    if attr.value.starts_with("#") {
                        return Some(element.content.to_string());
                    }

                    // For relative links, guess the page it points to.
                    let link = if attr.value.starts_with("/") {
                        format!("https://bazel.build{}", attr.value.to_string())
                    } else if attr.value.starts_with("../") {
                        format!(
                            "https://bazel.build/rules/lib/{}",
                            attr.value.strip_prefix("../").unwrap()
                        )
                    } else {
                        // For absolute links, just use the link.
                        attr.value.to_string()
                    };

                    return Some(format!("[{}]({})", element.content.to_string(), link));
                }
            }
            Some(element.content.to_string())
        })
        .build()
});

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
        params: DocParams {
            named_only: rule
                .attribute
                .iter()
                .map(|attribute| DocParam {
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
        },
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
    let docs = create_docstring_for_possible_html(&value.doc);

    if let Some(callable) = &value.callable {
        let mut params = DocParams {
            ..Default::default()
        };

        for param in callable.param.iter() {
            let name = if param.is_star_arg {
                param.name.strip_prefix('*').unwrap_or(&param.name)
            } else if param.is_star_star_arg {
                param.name.strip_prefix("**").unwrap_or(&param.name)
            } else {
                &param.name
            }
            .to_string();

            let doc_param = DocParam {
                name,
                docs: create_docstring_for_possible_html(&param.doc),
                typ: Ty::any(),
                default_value: if param.is_mandatory {
                    None
                } else {
                    Some(param.default_value.clone())
                },
            };

            if param.is_star_arg {
                params.args = Some(doc_param);
            } else if param.is_star_star_arg {
                params.kwargs = Some(doc_param);
            } else {
                // Most of bazel builtins have positional only parameters, but this information is
                // not available in proto, to err on safe side adding all params to positional or
                // named.
                params.pos_or_named.push(doc_param);
            }
        }

        DocMember::Function(DocFunction {
            docs,
            params,
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

fn create_docstring_for_possible_html(html: &str) -> Option<DocString> {
    // Some documentation is using markdown, use simple heuristic to check whether need to convert
    // from html to markdown.
    let markdown = if str::contains(html, "<") {
        match HTML_CONVERTER.convert(html).ok() {
            Some(markdown) => markdown,
            None => html.to_string(),
        }
    } else {
        html.to_string()
    };

    create_docstring(&markdown)
}
