pub use builtin_proto::builtin::*;
use starlark::{docs::{DocFunction, DocMember, DocProperty, DocString}, typing::Ty};

pub fn value_to_doc_member(value: &Value) -> DocMember {
    let docs = Some(DocString { summary: value.doc.clone(), details: None });

    if let Some(callable) = &value.callable {
        DocMember::Function(DocFunction { docs, ..Default::default() })
    } else {
        DocMember::Property(DocProperty { docs, typ: Ty::any() })
    }
}