#[macro_use]
extern crate criterion;

use criterion::Criterion;
use inventory_mgt::*;
use std::process::Command;

fn create_command() -> Command {
    let mut command = Command::new("python3");
    command.arg("tests/external_process.py");
    command
}

fn benchmark_update_rs(c: &mut Criterion) {
    c.bench_function("test update with rust", move |b| {
        b.iter(|| {
            update_master("MasterInventory.csv".to_string(), "SupplyInventory.csv".to_string()).unwrap();

        })
    });
}

#[allow(deprecated)]
fn benchmark_update_py(c: &mut Criterion) {
    c.bench_function("test update with python", move |b| {
        b.iter(|| {
            create_command();
        })
    });
}


criterion_group!(
    benches,
    benchmark_update_rs,
    benchmark_update_py,
);
criterion_main!(benches);
