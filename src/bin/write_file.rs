use std::fs::OpenOptions;
use std::io::*;
use std::time::SystemTime;

use chrono::prelude::*;
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    const IP_POOL_SIZE: usize = 100;
    let mut ip_pool: Vec<String> = Vec::new();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("log.txt")?;
    let start = SystemTime::now();

    let mut buf_writer = BufWriter::with_capacity(1024 * 256, file);
    let mut random_gen = thread_rng();

    for _ in 0..IP_POOL_SIZE {
        let ip_sec_1 = random_gen.gen_range(1, 256);
        let ip_sec_2 = random_gen.gen_range(1, 256);
        let ip_sec_3 = random_gen.gen_range(1, 256);
        let ip_sec_4 = random_gen.gen_range(1, 256);
        let ip = format!("{}.{}.{}.{}", ip_sec_1, ip_sec_2, ip_sec_3, ip_sec_4);
        ip_pool.push(ip);
    }

    for _ in 0..10000 * 10000 {
        let time = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let ip_idx = random_gen.gen_range(0, IP_POOL_SIZE);
        let ip = &ip_pool[ip_idx];
        let resp_time = random_gen.gen_range(1, 200);

        let line = format!("{}, {}, 200, {}\n", time, ip, resp_time);
        buf_writer.write_all(line.as_bytes())?;
    }
    buf_writer.flush()?;

    let duration = start.elapsed().unwrap();
    println!("{}", duration.as_millis());

    Ok(())
}
