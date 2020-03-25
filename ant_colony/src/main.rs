mod colony;
mod job_list;
mod ants;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const CHOSEN_PROBLEM : usize = 0;
const SINGLE : bool = false;

fn main() {
    if SINGLE {
        let jobs = read_input(format!("test_data/{}.txt", CHOSEN_PROBLEM + 1).as_str());
        let solution = colony::run(&jobs);
        output(format!("test_data/{}_solution.txt", CHOSEN_PROBLEM + 1).as_str(), solution).expect("Problem");
    }
    else {
        for i in 0..7 {
            println!("Problem {}", i);
            let jobs = read_input(format!("test_data/{}.txt", i + 1).as_str());
            let solution = colony::run(&jobs);
            output(format!("test_data/{}_solution.txt", i + 1).as_str(), solution).expect("Problem");
        }
    }
}

fn read_input(path : &str) -> job_list::Jobs {
    let path = Path::new(path);
	
    let mut s = String::new();
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    match file.read_to_string(&mut s) {
        Err(_) => panic!("Oops"),
        _ => (),
    }

    let mut lines = s.lines();
    let num : Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| usize::from_str_radix(s, 10).unwrap()).collect();
    let jobs = num[0];
    let machines = num[1];
    let mut init_vec = Vec::new();
    for job in lines {
        let num_pairs : Vec<usize> = job.split_whitespace().map(|s| usize::from_str_radix(s, 10).unwrap()).collect();
        for i in 0..num_pairs.len()/2 {
            init_vec.push((num_pairs[2*i], num_pairs[2*i+1]));
        }
    }
    return job_list::Jobs::init(init_vec, machines, jobs);
}

fn output(path : &str, ord : job_list::Ordering) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let result_string = ord.output();
    file.write_all(result_string.as_bytes())?;
    Ok(())
}