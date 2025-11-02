fn split_file(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut iter = input.splitn(2, "\n\n");
    let rules = iter.next().unwrap_or("");
    let orders = iter.next().unwrap_or("");


    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in rules.lines() {
        let mut parts = rule.split('|');
        let key: i32 = parts.next().unwrap().trim().parse().unwrap();
        let value: i32 = parts.next().unwrap().trim().parse().unwrap();

        rules_map.entry(key).or_insert_with(HashSet::new).insert(value);
    }

    let orders: Vec<Vec<i32>> = orders
        .lines()
        .map(|line| {
            line.split(',')
            .filter_map(|num| num.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>()
        })
        .collect();

    (rules_map, orders)
}

fn valid_update(rules: &HashMap<i32, HashSet<i32>>, orders: &Vec<Vec<i32>>) -> i32 {
    let mut ans_vec: Vec<i32> = Vec::new();
    for row in orders {
        let mut valid = true;
        // println!("{:?}", row);
        for i in 1..row.len() {
            let current_num = row[i];
            let priors = &row[0..i];
            for &prior_num in priors {
               if let Some(rule) = rules.get(&prior_num){
                    if !rule.contains(&current_num){
                        valid = false;
                        break;
                    }
                }    
            }
            if !valid {
                break;
            }
        }
        if valid {
            let mid = row[row.len() / 2];
            ans_vec.push(mid);
        }
    }
    ans_vec.iter().sum()
}
fn invalid_to_valid(rules: HashMap<i32, HashSet<i32>>, orders: Vec<Vec<i32>>) -> i32 {
    let mut invalid_ans_vec: Vec<i32> = Vec::new();
    for row in orders {
        let mut valid = true;
        // println!("{:?}", row);
        for i in 1..row.len() {
            let current_num = row[i];
            let priors = &row[0..i];
            for &prior_num in priors {
               if let Some(rule) = rules.get(&prior_num){
                    if !rule.contains(&current_num){
                        valid = false;
                        break;
                    }
                }    
            }
            if !valid {
                let sorted: Vec<i32> = invalid_sort(&row, &rules);

                let mid = sorted[sorted.len() / 2];
                invalid_ans_vec.push(mid);
                break;

            }
        }
       
    }
    invalid_ans_vec.iter().sum()
}
fn invalid_sort(order: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    // set of rules for just this row
    let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
    // number and amount of nodes(numbers) that must come before it
    // Indegrees:

    let mut indegree: HashMap<i32, usize> = HashMap::new();

    // Initialize indegree with every number from order to make sure numbers with no rules are added
    for &n in order {
        indegree.entry(n).or_insert(0);
    }

    // Adds number of dependencies for each number (numbers that must come before it in the order)
    // 97: 0
    // 61: 2   // 97 and 13 must come before
    // 13: 1   // 97 must come before 
    for &a in order {
        if let Some(afters) = rules.get(&a) {
            for &b in afters {
                if order.contains(&b) {
                    adj.entry(a).or_insert_with(HashSet::new).insert(b);
                    *indegree.entry(b).or_insert(0) += 1;
                }
            }
        }
    }

    // Kahn's topological sort
    let mut queue: VecDeque<i32> = indegree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&n, _)| n)
        .collect();

    let mut sorted = Vec::new();

    while let Some(n) = queue.pop_front() {
        sorted.push(n);
        if let Some(children) = adj.get(&n) {
            for &m in children {
                if let Some(indeg) = indegree.get_mut(&m) {
                    *indeg -= 1;
                    if *indeg == 0 {
                        queue.push_back(m);
                    }
                }
            }
        }
    }

    // If successful, return sorted list; otherwise, fall back
    if sorted.len() == order.len() {
        sorted
    } else {
        order.clone()
    }
}

use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("not read");

    let (rules, orders) = split_file(&input);
    // println!("{:?}", &rules);
    let result = valid_update(&rules, &orders);
    println!("valid: {}", result);
    let result = invalid_to_valid(rules, orders);
    println!("invalid to valid: {}", result);
}
