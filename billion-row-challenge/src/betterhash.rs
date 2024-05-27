use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    slice::Chunks,
};

use rustc_hash::FxHashMap;

struct Station {
    total: i32,
    sum: f64,
    max: f64,
    min: f64,
}

impl Default for Station {
    fn default() -> Self {
        Self {
            total: 0,
            sum: 0.0,
            max: f64::NEG_INFINITY,
            min: f64::INFINITY,
        }
    }

}



pub struct BetterHash;

impl BetterHash {
    pub fn start(path: &'static str) {
        let file = File::open(path).unwrap();

        let chunk_size = 4096;
        let mut reader = BufReader::new(file);

        let mut results: FxHashMap<String, Station> = FxHashMap::default();

        let mut buffer = String::new();

        loop {
            let len = reader.read_line(&mut buffer).unwrap();
            if len == 0 {
                break;
            }
            let line = get_parts(&buffer);
            results
                .entry(line[0].to_string())
                .and_modify(|station| update_station(station, line[1].trim()))
                .or_insert(Station::default());
            buffer.clear();
        }

        for (key, value) in results {
            println!(
                "{};{:.1};{:.1};{:.1}",
                key,
                value.min,
                value.sum / value.total as f64,
                value.max
            );
        }
    }
}

fn update_station(station: &mut Station, data: &str) {
    let float = data.parse::<f64>().unwrap();
    station.total += 1;
    station.sum += float;
    station.max = station.max.max(float);
    station.min = station.min.min(float);
}

#[inline]
fn get_parts(string: &String) -> Vec<&str> {
    string.split(";").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        BetterHash::start("measurements.txt");
    }
}
