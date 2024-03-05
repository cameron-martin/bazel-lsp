pub use builtin_proto::builtin::*;
use starlark::{
    docs::{DocFunction, DocMember, DocParam, DocProperty, DocString},
    typing::{Ty, TyStruct},
};

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
