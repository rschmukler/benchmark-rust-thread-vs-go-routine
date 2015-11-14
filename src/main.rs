extern crate time;

use std::env;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
  if let Some(benchmark_string) = env::args().nth(1) {
    let benchmark_count = benchmark_string.parse::<u32>().unwrap();
    let (tx, rx) = channel();
    for _ in 0..benchmark_count {
      let tx = tx.clone();
      let start = time::precise_time_ns() as f64;
      let child = thread::spawn(move || {
        let end = time::precise_time_ns() as f64;
        let sample = (end - start) / (1000 as f64);
        println!("  sample: {}ms", sample);
        tx.send(sample).unwrap();
      });
      child.join().unwrap()
    }
    let average = rx.iter().take(benchmark_count as usize).fold(0 as f64, | total, sample| {
        total + sample / (benchmark_count as f64)
    });
    println!("Average sample: {}ms", average);
  } else {
    println!("Please provide a benchmark count");
  }
}
