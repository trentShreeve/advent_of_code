
use std::fs;

fn split_whitespace(contents: &str) -> (Vec<i64>, Vec<i64>) {
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

fn total_distance(left: Vec<i64>, right: Vec<i64>) -> i64 {
    let mut dist: Vec<i64> = Vec::new();
    for (a, b) in left.iter().zip(right.iter()) {
        let distance = a - b;
        dist.push(distance.abs());   
    }
    dist.iter().sum()
}


fn similarity_score(left: Vec<i64>, right: Vec<i64>) -> i64 {
    let mut int_score: Vec<i64> = Vec::new();
    
    for a in left.iter() {
        // println!("a: {}",a);
        let mut count = 0;
        for b in right.iter() {
            if a == b {
                count+=1;    
            }
        }
        int_score.push(count * a);
    }
    int_score.iter().sum()
}

fn main() {

    // PART ONE
    // reading file
    let contents = fs::read_to_string("input.txt").expect("couldn't read");
    
    // // parse file into two vectors and sort
    let (left, right) = split_whitespace(&contents);


    // get the distances betrween each index in the list and sum total distance
    let total = total_distance(left, right);
    println!("{:?}", total);

    //PART TWO
    let (left, right) = split_whitespace(&contents);
    let score = similarity_score(left, right);
    println!("{:?}", score);
    

}

