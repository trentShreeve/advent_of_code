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

// let left = cap.get(1)?.as_str().parse::<i32>().ok()?;
        // let right = cap.get(2)?.as_str().parse::<i32>().ok()?;
        // let product = left * right;
        // Some(product.to_string())
fn pt2_correct_memory (contents: &str) -> i32 {
    let re = Regex::new(r"(?P<dont>don't())|(?P<do>do())|mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    
    // let filtered: Vec<String> = re.captures_iter(contents)
    //     .filter_map(|cap| {
    //         cap.name("do").map(|_| "true".to_string())
    //         .or_else(|| cap.name("dont").map(|_| "false".to_string()))
    //         .or_else(|| {
    //             cap.name("a").and_then(|a| cap.name("b").and_then(|b| {
    //                 a.as_str().parse::<i32>().ok().zip(b.as_str().parse::<i32>().ok())
    //                     .map(|(a, b)| (a * b).to_string())
    //             }))
    //         })
    //     }).collect();

    let filtered: Vec<String> = re.captures_iter(contents)
        .filter_map(|cap| { 
        match (cap.name("do"), cap.name("dont"), cap.name("a"), cap.name("b")) {
            (Some(_), None, None, None,) => Some("true".to_string()),
            (None, Some(_), None, None,) => Some("false".to_string()),
            (None, None, Some(a), Some(b)) => {
                a.as_str().parse::<i32>().ok().zip(b.as_str().parse::<i32>().ok())
                .map(|(a, b)| (a * b).to_string())
            }
            _ => None,
        }
    }).collect();
    // println!("{:?}", filtered);
    let mut switch = true;
    let mut total = 0;
    for item in filtered.iter() {
        match item.as_str() {
            "true" => switch = true,
            "false" => switch = false,
            _ => {
                if switch {
                    if let Ok(num) = item.parse::<i32>() {
                        total += num;
                    }
                }
            }
        }
    }
    total    
}
fn main() {
    

    let contents = fs::read_to_string("input.txt").expect("failed read");
    // println!("{:?}", contents);
    
    // establish what right looks like: mul(num,num)
    let corrected_memory = correct_memory(&contents);
    println!("pt1: {:?}", corrected_memory);
    let pt2 = pt2_correct_memory(&contents);
    println!("pt2: {:?}", pt2);
}
