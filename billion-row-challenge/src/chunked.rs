use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    slice::Chunks,
};

const BUFSIZE: usize = (1 << 20) * 32;

#[derive(Default)]
struct Station {
    total: i32,
    sum: f64,
    max: f64,
    min: f64,
}

pub struct Chunked;

impl Chunked {
    pub fn start(path: &'static str) {
        let file = File::open(path).unwrap();

        let chunk_size = 4096;

        let mut results: HashMap<, Station> = HashMap::new();

        let mut buffer = [0; BUFSIZE];

        loop {
            let len = file.read(&mut buffer).unwrap();
            if len == 0 {
                break;
            }

            let line = get_parts(&buffer);
            results
                .entry(line[0].to_string())
                .and_modify(|station| update_station(station, line[1].trim()))
                .or_insert(Station::default());
            
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

fn parse_buffer(buffer: &[u8; BUFSIZE], map: &HashMap<String, Station>) -> [u8; 100] {
    
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
        Chunked::start("measurements.txt");
    }
}
