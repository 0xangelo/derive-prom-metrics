use darling::FromField as _;
use syn::spanned::Spanned as _;
use syn::{parse_quote, Error, Field, Ident, Result, TypePath};

/// The types of Prometheus metrics supported.
#[derive(Debug)]
pub enum MetricType {
    Counter,
    Gauge,
    IntCounter,
    IntGauge,
    Histogram,

    CounterVec,
    GaugeVec,
    IntCounterVec,
    IntGaugeVec,
    HistogramVec,
}

/// Identify the Prometheus metric type from the struct field type.
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
            "Counter" => Self::Counter,
            "Gauge" => Self::Gauge,
            "IntCounter" => Self::IntCounter,
            "IntGauge" => Self::IntGauge,
            "Histogram" => Self::Histogram,

            "CounterVec" => Self::CounterVec,
            "GaugeVec" => Self::GaugeVec,
            "IntCounterVec" => Self::IntCounterVec,
            "IntGaugeVec" => Self::IntGaugeVec,
            "HistogramVec" => Self::HistogramVec,
            _ => return Err(Error::new(ident.span(), "Invalid type")),
        })
    }
}

impl MetricType {
    /// Rust expression for initializing a field of this type and adding it to a registry.
    pub fn init_expr(&self, ident: Ident, field: &Field) -> Result<syn::Expr> {
        use MetricType::*;

        let name = ident.to_string();
        let field_opts = crate::opts::Field::from_field(field)?;

        self.pre_validate(&field_opts)
            .map_err(|e| e.with_span(field))?;

        let help = field_opts.metric_help()?;

        let ident_ = self.to_ident();
        Ok(match self {
            Counter | Gauge | IntCounter | IntGauge => {
                parse_quote!(::prometheus::#ident_::new(#name, #help)?)
            }

            Histogram => {
                let buckets = field_opts.buckets.ok_or_else(|| {
                    darling::Error::custom(
                        "#[prometheus(buckets = ...)] required for Histogram metric type",
                    )
                    .with_span(field)
                })?;
                parse_quote!(::prometheus::#ident_::with_opts({
                    let hopts = ::prometheus::HistogramOpts::new(#name, #help);
                    hopts.buckets(#buckets)
                })?)
            }

            CounterVec | GaugeVec | IntCounterVec | IntGaugeVec => {
                let label_names = field_opts.label_names.ok_or_else(|| {
                    darling::Error::custom(
                        "#[prometheus(label_names = ...)] required for Vec metric types",
                    )
                    .with_span(field)
                })?;
                parse_quote!(::prometheus::#ident_::new(
                    ::prometheus::core::Opts::new(#name, #help),
                    #label_names,
                )?)
            }

            HistogramVec => {
                let buckets = field_opts.buckets.ok_or_else(|| {
                    darling::Error::custom(
                        "#[prometheus(buckets = ...)] required for Histogram metric type",
                    )
                    .with_span(field)
                })?;
                let label_names = field_opts.label_names.ok_or_else(|| {
                    darling::Error::custom(
                        "#[prometheus(label_names = ...)] required for Vec metric types",
                    )
                    .with_span(field)
                })?;
                parse_quote!(::prometheus::#ident_::new(
                    {
                        let hopts = ::prometheus::HistogramOpts::new(#name, #help);
                        hopts.buckets(#buckets)
                    },
                    #label_names,
                )?)
            }
        })
    }

    /// Make sure there are no attributes to a field that do not apply to it based on its metric
    /// type.
    fn pre_validate(&self, opts: &crate::opts::Field) -> darling::Result<()> {
        use MetricType::*;

        if opts.buckets.is_some() && !matches!(self, Histogram | HistogramVec) {
            return Err(darling::Error::custom(format!(
                "#[prometheus(buckets = ...)] is only for histogram metric types, not {self:?}"
            )));
        }

        if opts.label_names.is_some()
            && !matches!(
                self,
                CounterVec | GaugeVec | IntCounterVec | IntGaugeVec | HistogramVec
            )
        {
            return Err(darling::Error::custom(format!(
                "#[prometheus(label_names = ...)] is only for Vec metric types, not {self:?}"
            )));
        }

        Ok(())
    }

    /// The identifier of the Prometheus Rust type.
    fn to_ident(&self) -> syn::Ident {
        use MetricType::*;

        match self {
            Counter => parse_quote!(Counter),
            Gauge => parse_quote!(Gauge),
            IntCounter => parse_quote!(IntCounter),
            IntGauge => parse_quote!(IntGauge),
            Histogram => parse_quote!(Histogram),

            CounterVec => parse_quote!(CounterVec),
            GaugeVec => parse_quote!(GaugeVec),
            IntCounterVec => parse_quote!(IntCounterVec),
            IntGaugeVec => parse_quote!(IntGaugeVec),
            HistogramVec => parse_quote!(HistogramVec),
        }
    }
}
