use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use rand::prelude::*;
use std::collections::{VecDeque, HashSet};

fn main() {
    let path = "facebook_combined.txt";

    let edges = read_file(path);
    let adj_list = adjacency_list(edges);
   
    //println!("{:?}", edges);
    //println!("{:?}", adj_list);
   
    let path = "facebook_combined.txt";
    match read_file(path) {
        Ok(edges) => {
        let adj_list = adjacency_list(edges);
        println!("{:?}", adj_list);

        let start_vertex = 0; // Choose the start vertex here
        let bfs_order = bfs(&adj_list, start_vertex);
        println!("BFS order from vertex {}: {:?}", start_vertex, bfs_order);
        }
    Err(err) => eprintln!("Error reading file: {}", err),
    }
}



//reading the file
fn read_file(file_path: &str) -> Vec<(usize,usize)> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut graph: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        graph.push((u, v));
    }
    graph
}

fn adjacency_list(edges: Vec<(usize,usize)>) -> Vec<Vec<usize>> {
    let mut adj: Vec<Vec<usize>> = vec![vec![];4039];
    for (i, j) in edges.iter(){
        adj[*i].push(*j);
    };
    return adj

}
// make breadth first search
fn bfs(adj_list: &[Vec<usize>], start_vertex: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut bfs_order = Vec::new();

    queue.push_back(start_vertex);
    visited.insert(start_vertex);

    while let Some(vertex) = queue.pop_front() {
        bfs_order.push(vertex);
        for &neighbor in &adj_list[vertex] {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    bfs_order
}
