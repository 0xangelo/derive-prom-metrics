#![crate_type = "proc-macro"]

use darling::FromField as _;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned as _;
use syn::{DeriveInput, Error, Result, TypePath};

mod opts;

#[proc_macro_derive(Metrics, attributes(prometheus))]
pub fn metrics_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_metrics_new(item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn impl_metrics_new(item: TokenStream) -> Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(item)?;

    let syn::Data::Struct(data) = ast.data else {
        return Err(Error::new(ast.span(), "Metrics only derived for structs"));
    };

    let syn::Fields::Named(fields) = data.fields else {
        return Err(Error::new(
            data.fields.span(),
            "Tuple or unit structs aren't supported",
        ));
    };

    let struct_name = ast.ident;

    let field_inits = field_initializers(fields)?;

    Ok(quote! {
        impl #struct_name {
            pub fn new(registry: &::prometheus::Registry) -> ::prometheus::Result<Self> {
                Ok(Self {
                    #(#field_inits),*
                })
            }
        }
    })
}

fn field_initializers(fields: syn::FieldsNamed) -> Result<Vec<TokenStream>> {
    let mut result = vec![];

    for field in fields.named {
        let syn::Type::Path(ref ty) = field.ty else {
            return Err(Error::new(
                field.ty.span(),
                format!("Field type '{:?}' unsupported", field.ty),
            ));
        };

        let metric_type: MetricType = ty.try_into()?;

        result.push(field_initializer(&field, metric_type)?);
    }

    Ok(result)
}

fn field_initializer(field: &syn::Field, metric_type: MetricType) -> Result<TokenStream> {
    use MetricType::*;

    let ident = field
        .ident
        .clone()
        .ok_or_else(|| Error::new(field.span(), "Field must be named"))?;

    let metric_name = ident.to_string();
    let metric_help = self::opts::Field::from_field(field)?.metric_help()?;

    let initializer = match metric_type {
        Gauge => quote!({
            let gauge = ::prometheus::Gauge::new(#metric_name, #metric_help)?;
            registry.register(Box::new(gauge.clone()))?;
            gauge
        }),
    };

    Ok(quote! {
        #ident: #initializer
    })
}

enum MetricType {
    Gauge,
    // TODO: add other types
}

impl TryFrom<&TypePath> for MetricType {
    type Error = Error;

    fn try_from(value: &TypePath) -> Result<Self> {
        use syn::{Path, PathArguments, PathSegment};

        let segments = match &value {
            TypePath {
                qself: None,
                path: Path { segments, .. },
            } => segments,

            _ => {
                return Err(Error::new(
                    value.span(),
                    format!("Field of type {value:?} not supported"),
                ))
            }
        };

        let ident = match segments.last() {
            None => return Err(Error::new(value.span(), "Missing last field type segment")),
            Some(PathSegment {
                ident,
                arguments: PathArguments::None,
            }) => ident,
            Some(other) => return Err(Error::new(other.span(), "Invalid type structure")),
        };

        Ok(match ident.to_string().as_str() {
            "Gauge" => Self::Gauge,
            _ => return Err(Error::new(ident.span(), "Invalid type")),
        })
    }
}
