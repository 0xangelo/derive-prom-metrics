//! Build [Prometheus] metrics declaratively as structs.
//!
//! This crate is in the very early stages of development.
//!
//! [Prometheus]: prometheus
//!
//! # Example
//!
//! ```no_run
//! use derive_prom_metrics::Metrics;
//! use prometheus::{linear_buckets, Gauge};
//!
//! #[derive(Metrics, Debug)]
//! struct Metrics {
//!     /// A simple counter.
//!     counter: prometheus::Counter,
//!
//!     /// A simple gauge.
//!     /// Is this in the same line? No.
//!     ///
//!     /// This will make it into the help text.
//!     gauge: Gauge,
//!
//!     /**
//!      * My help
//!      *
//!      * This will make it into the help text, as well as all of the leading asterisks.
//!      */
//!     int_counter: prometheus::IntCounter,
//!
//!     /// An integer gauge.
//!     int_gauge: prometheus::IntGauge,
//!
//!     /// A histogram.
//!     #[prometheus(buckets = linear_buckets(0.005, 0.005, 999)?)]
//!     histogram: prometheus::Histogram,
//!
//!     /// A vector of counters, one for each label.
//!     #[prometheus(label_names = &["label"])]
//!     counter_vec: prometheus::CounterVec,
//!
//!     /// A vector of gauges, one for each label.
//!     #[prometheus(label_names = &["label"])]
//!     gauge_vec: prometheus::GaugeVec,
//!
//!     /// A vector of integer counters, one for each label.
//!     #[prometheus(label_names = &["label"])]
//!     int_counter_vec: prometheus::IntCounterVec,
//!
//!     /// A vector of integer gauges, one for each label.
//!     #[prometheus(label_names = &["label"])]
//!     int_gauge_vec: prometheus::GaugeVec,
//!
//!     /// A vector of histograms, one for each label.
//!     #[prometheus(
//!         buckets = linear_buckets(0.005, 0.005, 999)?,
//!         label_names = &["label"],
//!     )]
//!     histogram_vec: prometheus::HistogramVec,
//! }
//! ```
#![crate_type = "proc-macro"]

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned as _;
use syn::{DeriveInput, Error, Result};

mod metric_type;
mod opts;

use self::metric_type::MetricType;

#[proc_macro_derive(Metrics, attributes(prometheus))]
pub fn metrics_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_metrics_new(item.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// The `new` method implementation for the metrics struct.
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
        result.push(field_initializer(&field)?);
    }

    Ok(result)
}

fn field_initializer(field: &syn::Field) -> Result<TokenStream> {
    let syn::Type::Path(ref ty) = field.ty else {
        return Err(Error::new(
            field.ty.span(),
            format!("Field type '{:?}' unsupported", field.ty),
        ));
    };

    let metric_type: MetricType = ty.try_into()?;

    let ident = field
        .ident
        .clone()
        .ok_or_else(|| Error::new(field.span(), "Field must be named"))?;

    let metric_init = metric_type.init_expr(ident.clone(), field)?;

    Ok(quote! {
        #ident: {
            let metric = #metric_init;
            registry.register(Box::new(metric.clone()))?;
            metric
        }
    })
}
