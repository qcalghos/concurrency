use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};
const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::new();
    println!("{:?}",metrics.snapshot());
    for idx in 0..N {
        task_worker(idx, metrics.clone());//Arc::clone(&metrics.data)
    }
    for _ in 0..M {
        request_worker(metrics.clone());
    }
    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{:?}",metrics.snapshot().unwrap());
    }
    // println!("{:?}", metrics.snapshot());
    
    Ok(())
}

fn task_worker(idx: usize,metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(100..1000)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    });
}
fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page)).unwrap();
        // metrics.inc("req.page.2");
        // metrics.inc("req.page.3");
    });
}
