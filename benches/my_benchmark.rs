#[macro_use]
extern crate criterion;

use criterion::Criterion;
use inventory_mgt::*;

fn benchmark_update_series(c: &mut Criterion) {
    c.bench_function("gen new master in serial", move |b| {
        b.iter(|| {
            update_master("MasterInventory.csv".to_string(), "SupplyInventory.csv".to_string()).unwrap();

        })
    });
}


criterion_group!(
    benches,
    benchmark_update_series,
);
criterion_main!(benches);
