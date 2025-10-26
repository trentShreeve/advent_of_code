use std::fs;
use regex::Regex;

// identify the correct memory and do calcs -> vec[answers]
fn correct_memory(contents: &str) -> i32 {
    
    let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    
    re.captures_iter(contents)
        .filter_map(|cap| {
        let left = cap.get(1)?.as_str().parse::<i32>().ok()?;
        let right = cap.get(2)?.as_str().parse::<i32>().ok()?;
        let product = left * right;
        Some(product)
        }).sum()
}

fn main() {
    

    let contents = fs::read_to_string("pt2_example.txt").expect("failed read");
    // println!("{:?}", contents);
    
    // establish what right looks like: mul(num,num)
    let corrected_memory = correct_memory(&contents);
    println!("{:?}", corrected_memory);
}
