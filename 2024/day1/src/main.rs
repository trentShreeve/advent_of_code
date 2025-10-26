// day one
use std::fs;

fn split_whitespace_sort(contents: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in contents.lines() {
        let nums: Vec<i64> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

        if nums.len() >= 2 {
            left.push(nums[0]);
            right.push(nums[1]);
        }
    }
    // sort the vectors
    left.sort();
    right.sort();
    (left, right)
}

fn total_distance(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    left.iter()
    .zip(right.iter())
    .map(|(a,b)| (a-b).abs())
    .sum()
}

use::std::collections::HashMap;

fn similarity_score(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    let mut score = HashMap::new();
    for b in right {
        *score.entry(b).or_insert(0) += 1;
    }
    left.iter()
    .map(|a| a * score.get(a).unwrap_or(&0))
    .sum()
}

fn main() {

    // reading file
    let contents = fs::read_to_string("example.txt").expect("couldn't read");
    
    // // parse file into two vectors and sort
    let (left, right) = split_whitespace_sort(&contents);

    // PART ONE
    // get the distances betrween each index in the list and sum total distance
    let total = total_distance(&left, &right);
    println!("total distance: {:?}", total);

    //PART TWO
    let score = similarity_score(&left, &right);
    println!("similarity score: {:?}", score);
    

}
