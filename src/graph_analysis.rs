use std::fs::File;
use std::collections::HashMap;
use rand::prelude::*;
use std::collections::{VecDeque, HashSet};
use std::io::{BufReader, BufRead, Error};

// Now we make an adjacency list 
pub fn adjacency_list(edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut max_node = 0;
    
    //let mut adj: Vec<Vec<usize>> = vec![vec![];30];
    for (i, j) in edges.iter(){
        max_node = max_node.max(*i).max(*j);
    }
    let mut adj: Vec<Vec<usize>> = vec![vec![]; max_node + 1];
    for (i, j) in edges.iter() {
        adj[*i].push(*j);
    }
    adj
}
// make breadth first search
pub fn bfs(adj_list: &[Vec<usize>], start_vertex: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut bfs_order = Vec::new();

    queue.push_back(start_vertex);
    visited.insert(start_vertex);

    while let Some(vertex) = queue.pop_front() {
        bfs_order.push(vertex);
        if let Some(neighbors) = adj_list.get(vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
            }
        }
    }else {
        eprintln!("Vertex index {} is out of bounds!", vertex);
    }
}

    bfs_order
}
// Now let's gather some insights about friends of friends.
// Function to find mutual friends up to a certain degree
pub fn find_mutual_friends(
    user_id: usize,
    adj_list: &[Vec<usize>],
    degree: usize,
) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut mutual_friends = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((user_id, 0));
    visited.insert(user_id);

    while let Some((user, current_degree)) = queue.pop_front() {
        if current_degree > degree {
            continue;
        }
        if current_degree > 0 {
            mutual_friends.insert(user);
        }

        if let Some(friends) = adj_list.get(user) {
            for &friend in friends {
                if !visited.contains(&friend) {
                    visited.insert(friend);
                    queue.push_back((friend, current_degree + 1));
                }
            }
        }
    }

    mutual_friends
}
//Now the main function will use the mutual friends info 