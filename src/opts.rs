use darling::{Error, FromField, Result};
use itertools::Itertools as _;
use syn::{Attribute, Expr, ExprLit, Ident, Lit, Meta, MetaNameValue};

/// Data extracted from the fields of the input struct.
#[derive(Debug, Clone, FromField)]
#[darling(attributes(prometheus), forward_attrs(doc))]
pub struct Field {
    attrs: Vec<Attribute>,
    ident: Option<Ident>,

    /// For histograms only.
    pub buckets: Option<Expr>,

    /// For `Vec_` metrics.
    pub label_names: Option<Expr>,
}

impl Field {
    pub fn metric_help(&self) -> Result<String> {
        let doc = self
            .attrs
            .iter()
            .filter_map(|a| match &a.meta {
                Meta::NameValue(MetaNameValue {
                    path,
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }),
                    ..
                }) if path.get_ident().is_some_and(|i| *i == "doc") => {
                    Some(lit_str.value().trim_ascii_start().to_owned())
                }
                _ => None,
            })
            .join("\n");

        if doc.is_empty() {
            return Err(Error::custom(
                "No doc for field; it is required to set a help message on the Prometheus metric",
            )
            .with_span(
                self.ident
                    .as_ref()
                    .expect("Only used with named struct fields"),
            ));
        }

        // Doc strings have a leading space usually
        Ok(doc)
    }
}
