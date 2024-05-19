use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    collections::HashMap,
    sync::atomic::Ordering,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}
impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}
impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, val) in self.data.iter() {
            writeln!(f, "{}:{}", key, val.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        //collect as hashmap
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        //collect::<HashMap<_,_>>
        AmapMetrics {
            data: Arc::new(map),
        }
    }
    pub fn inc(&mut self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Release);
        Ok(())
    }
    pub fn dec(&self, key: &str) -> Result<()> {
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key {} not dound", key))?;
        counter.fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}
