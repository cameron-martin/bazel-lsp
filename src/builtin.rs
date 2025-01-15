use std::sync::LazyLock;

pub use build_proto::blaze_query::*;
pub use builtin_proto::builtin::*;
use htmd::{Element, HtmlToMarkdown};
use starlark::{
    docs::{DocFunction, DocMember, DocParam, DocParams, DocProperty, DocString},
    typing::Ty,
};

use crate::file_type::FileType;

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

pub fn api_context_applicable_for_file_type(
    api_context: ApiContext,
    file_type: FileType) -> bool {
    match api_context {
        ApiContext::All => true,
        ApiContext::Bzl => file_type == FileType::Library,
        ApiContext::Build => file_type == FileType::Build,
        ApiContext::Module => file_type == FileType::Module,
        ApiContext::Repo => file_type == FileType::Repo,
        ApiContext::Vendor => file_type == FileType::Vendor,
        ApiContext::Workspace => file_type == FileType::Workspace,
    }
}

pub fn builtins_to_doc_members<'a>(
    globals: &'a Vec<Value>,
) -> impl Iterator<Item = (String, DocMember)> + 'a {
    globals.iter().flat_map(move |global| {
        Some((global.name.clone(), value_to_doc_member(global)))
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
