use derive_prom_metrics::Metrics;
use prometheus::{default_registry, linear_buckets, Gauge};

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

#[expect(clippy::unwrap_used, reason = "testing")]
fn main() {
    let registry = default_registry();
    let metrics = Metrics::new(registry).unwrap();
    println!("{:?}", metrics.counter);
    println!("{:?}", metrics.gauge);
    println!("{:?}", metrics.int_counter);
    println!("{:?}", metrics.int_gauge);
    println!("{:?}", metrics.histogram);
    println!("{:?}", metrics.counter_vec);
    println!("{:?}", metrics.gauge_vec);
    println!("{:?}", metrics.int_counter_vec);
    println!("{:?}", metrics.int_gauge_vec);
    println!("{:?}", metrics.histogram_vec);
}
