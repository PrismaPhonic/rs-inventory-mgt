#[macro_use]
extern crate criterion;

use criterion::Criterion;
use inventory_mgt::*;

fn benchmark_update_owned_string(c: &mut Criterion) {
    c.bench_function("test reading into owned String", move |b| {
        b.iter(|| {
            update_master("MasterInventory.csv".to_string(), "SupplyInventory.csv".to_string()).unwrap();

        })
    });
}

fn benchmark_update_str_ref(c: &mut Criterion) {
    c.bench_function("test reading into &str", move |b| {
        b.iter(|| {
            update_master_fast("MasterInventory.csv".to_string(), "SupplyInventory.csv".to_string()).unwrap();

        })
    });
}


criterion_group!(
    benches,
    benchmark_update_owned_string,
    benchmark_update_str_ref,
);
criterion_main!(benches);
