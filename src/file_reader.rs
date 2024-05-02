use std::fs::File;
use std::io::{BufReader, BufRead, Error};

//first let's read the file
pub fn read_file(file_path: &str) -> Result<Vec<(usize,usize)>, Error> {
    let file = File::open(file_path)?;//.expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        graph.push((u, v));
    }
    Ok(graph)
}