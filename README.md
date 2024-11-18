# derive-prom-metrics

[![Crates.io](https://img.shields.io/crates/v/derive-prom-metrics.svg)](https://crates.io/crates/derive-prom-metrics)
[![Docs.rs](https://docs.rs/derive-prom-metrics/badge.svg)](https://docs.rs/derive-prom-metrics)
[![CI](https://github.com/0xangelo/derive-prom-metrics/workflows/CI/badge.svg)](https://github.com/0xangelo/derive-prom-metrics/actions)
[![Rust GitHub Template](https://img.shields.io/badge/Rust%20GitHub-Template-blue)](https://rust-github.github.io/)

<!-- cargo-rdme start -->

Build [Prometheus] metrics declaratively as structs.

This crate is in the very early stages of development.

[Prometheus]: https://prometheus.io

## Example

```rust
use derive_prom_metrics::Metrics;
use prometheus::{linear_buckets, Gauge};

#[derive(Metrics, Debug)]
struct Metrics {
    /// A simple counter.
    counter: prometheus::Counter,

    /// A simple gauge.
    /// Is this in the same line? No.
    ///
    /// This will make it into the help text.
    gauge: Gauge,

    /**
     * My help
     *
     * This will make it into the help text, as well as all of the leading asterisks.
     */
    int_counter: prometheus::IntCounter,

    /// An integer gauge.
    int_gauge: prometheus::IntGauge,

    /// A histogram.
    #[prometheus(buckets = linear_buckets(0.005, 0.005, 999)?)]
    histogram: prometheus::Histogram,

    /// A vector of counters, one for each label.
    #[prometheus(label_names = &["label"])]
    counter_vec: prometheus::CounterVec,

    /// A vector of gauges, one for each label.
    #[prometheus(label_names = &["label"])]
    gauge_vec: prometheus::GaugeVec,

    /// A vector of integer counters, one for each label.
    #[prometheus(label_names = &["label"])]
    int_counter_vec: prometheus::IntCounterVec,

    /// A vector of integer gauges, one for each label.
    #[prometheus(label_names = &["label"])]
    int_gauge_vec: prometheus::GaugeVec,

    /// A vector of histograms, one for each label.
    #[prometheus(
        buckets = linear_buckets(0.005, 0.005, 999)?,
        label_names = &["label"],
    )]
    histogram_vec: prometheus::HistogramVec,
}
```

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Credits

Created using https://rust-github.github.io/.
