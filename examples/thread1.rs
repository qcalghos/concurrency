use anyhow::Result;
use std::{thread,sync::mpsc};

const NUM_PRODUCERS:usize=4;
fn main()->Result<()>{
    let (tx,rx)=mpsc::channel();
    for i in 0..NUM_PRODUCERS{
        let tx=tx.clone();
    }
   
    Ok(())
}