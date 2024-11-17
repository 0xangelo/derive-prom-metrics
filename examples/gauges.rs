use derive_prom_metrics::Metrics;
use prometheus::{default_registry, Gauge};

#[expect(dead_code)]
#[derive(Metrics, Debug)]
struct Metrics {
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
    another_gauge: Gauge,
}

#[expect(clippy::unwrap_used, reason = "testing")]
fn main() {
    let registry = default_registry();
    let metrics = Metrics::new(registry).unwrap();
    println!("{metrics:#?}");
}
