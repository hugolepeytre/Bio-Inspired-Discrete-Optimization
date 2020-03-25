const GENERATIONS : usize = 1000;
const ANTS : usize = 50;

use crate::job_list::{Jobs, Ordering};
use crate::ants::PheromoneMatrix;

use std::usize::MAX;
use rand::prelude::*;

pub fn run<'a>(jobs : &'a Jobs) -> Ordering<'a> {
    let mut best_solution : Ordering = Ordering::random(&jobs);
    let mut best_time = best_solution.end_time();
    let mut pheromones : PheromoneMatrix = PheromoneMatrix::init(jobs.n_machines(), jobs.n_jobs());
    for i in 0..GENERATIONS {
        let solutions : Vec<_> = (0..ANTS).map(|_| construct_solution(&pheromones, &jobs)).collect();
        let avg = solutions.iter().fold(0, |acc, sol| acc + sol.end_time())/ANTS;
        for s in solutions {
            pheromones.update_edges(&s, best_time);
            let end_time = s.end_time();
            if best_time > end_time {
                best_time = end_time;
                best_solution = s;
            }
        }
        println!("Gen {} : {}, {}", i + 1, best_time, avg);
    }
    return best_solution;
}

fn construct_solution<'a>(pheromones : &PheromoneMatrix, jobs : &'a Jobs) -> Ordering<'a> {
    let mut current_next_tasks = vec![0; jobs.n_jobs()];
    let mut job_order = Vec::new();
    let mut curr_pos = (MAX, MAX);
    for _ in 0..jobs.n_jobs()*jobs.n_machines() {
        curr_pos = choose_next_job(pheromones, &mut current_next_tasks, curr_pos, jobs.n_machines());
        job_order.push(curr_pos);
    }
    return Ordering::new(job_order, jobs);
}

fn choose_next_job(pheromones : &PheromoneMatrix, curr_next_tasks : &mut Vec<usize>, curr_pos : (usize, usize), n_machines : usize) -> (usize, usize) {
    let mut rd = thread_rng();
    let mut pheromone_values = Vec::new();
    let total = curr_next_tasks.into_iter().enumerate().fold(0.0, |acc, (job, &mut task)| {
        if task != n_machines {
            let p_v = pheromones.get_pheromones(curr_pos.0, job, curr_pos.1, task);
            pheromone_values.push(p_v); 
            acc + p_v
        }
        else {
            pheromone_values.push(0.0);
            acc
        }
    });
    let rand : f32 = rd.gen();
    let threshold : f32 = total * rand;
    let mut chosen_job = 0;
    let mut acc = 0.0;
    while acc <= threshold {
        if chosen_job == pheromone_values.len() {
            println!("Aie : threshold {} but total {}", threshold, total);
        }
        acc = acc + pheromone_values[chosen_job];
        chosen_job = chosen_job + 1;
    }
    chosen_job = chosen_job - 1;
    curr_next_tasks[chosen_job] = curr_next_tasks[chosen_job] + 1;
    return (chosen_job, curr_next_tasks[chosen_job] - 1)
}