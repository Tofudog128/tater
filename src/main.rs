use std::fs::File;
use std::collections::HashMap;
use rand::prelude::*;
use std::collections::{VecDeque, HashSet};
use std::io::{BufReader, BufRead, Error};
// Making some mods to hold my functions to keep things organized outside the main.
mod file_reader;
use crate::file_reader::read_file;
mod graph_analysis;
use crate::graph_analysis::adjacency_list;
use crate::graph_analysis::bfs;
use crate::graph_analysis::find_mutual_friends;

//Now the main function will use the mutual friends info 
fn main() {
    let path = "facebook_combined.txt";
    //Initialize adjacency list and edges within the main to ensure they are recognized
    let edges = match read_file(path) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let adj_list = adjacency_list(&edges);
    // The number of users is the length of the adj list
    let num_users = adj_list.len();

    // Calculate the number of edges to get the number of friendships
    let num_edges = edges.len();

    // Calculate the average number of friends per user using an average equation
    let avg_friends_per_user = (num_edges as f64 / num_users as f64).round() as usize;

    // Make new variables that will holf the user with the most friends and how many friends they have.
    let mut max_friends_count = 0;
    let mut max_friends_user_id = 0;

    // Iterate through all users to find the user with the most friends
    for (user_id, friends) in adj_list.iter().enumerate() {
        let num_friends = friends.len();
        if num_friends > max_friends_count {
            max_friends_count = num_friends;
            max_friends_user_id = user_id;
        }
    };

        let start_vertex = 6; // Choose the start vertex here
        let bfs_order = bfs(&adj_list, start_vertex);
        println!("BFS order from vertex {}: {:?}", start_vertex, bfs_order);
        // Print the user with the most friends
        println!("User with the most friends: {}", max_friends_user_id);
        println!("Number of friends: {}", max_friends_count);

        // Calculate and print the average number of friends per user
        println!("Average number of friends per user: {}", avg_friends_per_user);

        // Choose a user ID for any given user
        let user_id = 6; // Choose any user ID here

        // Find the first, second, and third-degree mutual friends for said user
        let first_degree_friends = find_mutual_friends(user_id, &adj_list, 1);
        let second_degree_friends = find_mutual_friends(user_id, &adj_list, 2);
        let third_degree_friends = find_mutual_friends(user_id, &adj_list, 3);

        // Calculate the total number of mutual friends
        let total_mutual_friends = first_degree_friends.len()
            + second_degree_friends.len()
            + third_degree_friends.len();

        // Calculate the percentage of the entire dataset represented by these mutual friends
        let percentage_of_dataset = (total_mutual_friends as f64 / num_users as f64) * 100.0;

        // Print the results
        println!("First-degree mutual friends: {:?}", first_degree_friends);
        println!("Second-degree mutual friends: {:?}", second_degree_friends);
        println!("Third-degree mutual friends: {:?}", third_degree_friends);
        println!("Percentage of the dataset represented by mutual friends: {:.2}%", percentage_of_dataset);
}

#[test]
fn test_mutual_1(){
    let path = "little_graph.txt";
    //Initialize adjacency list and edges within the main to ensure they are recognized
    let edges = match read_file(path) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let adj_list = adjacency_list(&edges);

        // Choose a user ID for any given user
        let user_id = 0; // Choose any user ID here

        // Find the first degree mutual friends for said user
        let first_degree_friends = find_mutual_friends(user_id, &adj_list, 1);
        let expected_friends: HashSet<usize> = vec![1, 3].into_iter().collect();
        assert_eq!(first_degree_friends, expected_friends);

        // Print the results
        println!("First-degree mutual friends: {:?}", first_degree_friends);
}
#[test]
fn test_mutual_2(){
    let path = "little_graph.txt";
    //Initialize adjacency list and edges within the main to ensure they are recognized
    let edges = match read_file(path) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let adj_list = adjacency_list(&edges);

        // Choose a user ID for any given user
        let user_id = 0; // Choose any user ID here

// Find the second degree mutuals for the user
        let second_degree_friends = find_mutual_friends(user_id, &adj_list, 2);
        let expected_friends: HashSet<usize> = vec![1, 3, 2, 8, 4, 5, 6].into_iter().collect();
        assert_eq!(second_degree_friends, expected_friends);

        // Print the results
        println!("Second-degree mutual friends: {:?}", second_degree_friends);
}
#[test]
fn test_mutual_3(){
    let path = "little_graph.txt";
    //Initialize adjacency list and edges within the main to ensure they are recognized
    let edges = match read_file(path) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let adj_list = adjacency_list(&edges);

        // Choose a user ID for any given user
        let user_id = 0; // Choose any user ID here

        // Find the third-degree mutual friends for said user
        let third_degree_friends = find_mutual_friends(user_id, &adj_list, 3);
        let expected_friends: HashSet<usize> = vec![1, 2, 4, 9, 7, 8, 3, 6, 5].into_iter().collect();
        assert_eq!(third_degree_friends, expected_friends);


        // Print the results
        println!("Third-degree mutual friends: {:?}", third_degree_friends);
}