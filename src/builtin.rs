pub use build_proto::blaze_query::*;
pub use builtin_proto::builtin::*;
use starlark::{
    docs::{DocFunction, DocMember, DocParam, DocProperty, DocString},
    typing::Ty,
};

use crate::file_type::FileType;

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
        docs: rule.documentation.as_ref().map(|doc| DocString {
            summary: doc.clone(),
            details: None,
        }),
        params: rule
            .attribute
            .iter()
            .map(|attribute| DocParam::Arg {
                name: attribute.name.clone(),
                docs: attribute.documentation.as_ref().map(|doc| DocString {
                    summary: doc.clone(),
                    details: None,
                }),
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

pub fn value_to_doc_member(value: &Value) -> DocMember {
    let docs = Some(DocString {
        summary: value.doc.clone(),
        details: None,
    });

    if let Some(callable) = &value.callable {
        DocMember::Function(DocFunction {
            docs,
            params: callable
                .param
                .iter()
                .map(|param| DocParam::Arg {
                    name: param.name.clone(),
                    docs: Some(DocString {
                        summary: param.doc.clone(),
                        details: None,
                    }),
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
