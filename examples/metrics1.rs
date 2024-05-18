use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::metrics1::Metrics;
use rand::Rng;
const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }
    for _ in 0..M {
        request_worker(metrics.clone());
    }
    loop {
        thread::sleep(Duration::from_secs(3));
        println!("{:?}", metrics.snapshot());
    }
}
fn task_worker(idx: usize, mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx));
    });
}

fn request_worker(mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(15..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page));
    });
}
