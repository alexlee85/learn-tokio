use std::collections::HashMap;
use std::fs::*;
use std::io::*;
use std::time::SystemTime;

fn main() -> Result<()> {
    let start = SystemTime::now();

    let file = OpenOptions::new()
        .read(true)
        .create(false)
        .write(false)
        .open("log2.txt")?;

    let buf_reader = BufReader::with_capacity(1024 * 256, file);

    let mut map: HashMap<String, u32> = HashMap::new();

    for line in buf_reader.lines() {
        let line_content: String = line.unwrap();
        let record: Vec<&str> = line_content.split(',').collect();

        let ip = record[1].trim().to_owned();
        *map.entry(ip).or_insert(0) += 1;
    }

    let mut ip_count_tuples: Vec<_> = map.iter().collect();
    ip_count_tuples.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:#?}", ip_count_tuples);

    // let ip_list: Vec<String> = buf_reader
    //     .lines()
    //     .map(|line| line.unwrap())
    //     .map(|x| {
    //         let ip: Vec<&str> = x.split(',').collect();
    //         ip[1].trim().to_owned()
    //     })
    //     .collect();

    // println!("{:?}", ip_list);

    // let map = count_ip(ip_list);
    // println!("{:#?}", map);

    let elapsed = start.elapsed().unwrap().as_millis();
    println!("time: {}", elapsed);

    Ok(())
}

#[allow(dead_code)]
fn count_ip(lines: &Vec<String>) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();

    lines.iter().for_each(|key| {
        *map.entry(key.to_owned()).or_insert(0) += 1;
    });

    map
}
