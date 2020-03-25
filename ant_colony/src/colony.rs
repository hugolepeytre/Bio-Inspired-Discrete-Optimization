const GENERATIONS : usize = 5000;
const ANTS : usize = 100;

use crate::job_list::{Jobs, Ordering};
use crate::ants::PheromoneMatrix;

use std::usize::MAX;
use rand::prelude::*;

pub fn run<'a>(jobs : &'a Jobs) -> Ordering<'a> {
    let mut best_solution : Ordering = Ordering::random(&jobs);
    let mut best_time = best_solution.end_time();
    let mut pheromones : PheromoneMatrix = PheromoneMatrix::init(jobs.n_machines(), jobs.n_jobs());
    for _ in 0..GENERATIONS {
        let solutions : Vec<_> = (0..ANTS).map(|_| construct_solution(&pheromones, &jobs)).collect();
        for s in solutions {
            pheromones.update_edges(&s);
            let end_time = s.end_time();
            if best_time > end_time {
                best_time = end_time;
                best_solution = s;
            }
        }
    }
    return best_solution;
}

fn construct_solution<'a>(pheromones : &PheromoneMatrix, jobs : &'a Jobs) -> Ordering<'a> {
    let solution : Vec<(usize, usize)> = (0..jobs.n_machines()).map(|machine_num| {
        let mut current_job = MAX;
        let mut jobs_left : Vec<usize> = (0..jobs.n_jobs()).collect();
        let mut job_order = Vec::new();
        while !jobs_left.is_empty() {
            current_job = choose_next_job(pheromones, current_job, &jobs_left, machine_num);
            let task_num = jobs.task_for_machine(current_job, machine_num);
            job_order.push((current_job, task_num));
            let p = jobs_left.iter().position(|&j| j == current_job).unwrap();
            jobs_left.remove(p);
        }
        job_order
    }).flatten().collect();
    return Ordering::new(solution, jobs);
}

fn choose_next_job(pheromones : &PheromoneMatrix, curr_job : usize, jobs_left : &Vec<usize>, machine_num : usize) -> usize {
    let mut rd = thread_rng();
    let mut pheromone_values = Vec::new();
    let total = jobs_left.into_iter().fold(0.0, |acc, &job| {
        let p_v = pheromones.get_pheromones(curr_job, job, machine_num);
        pheromone_values.push(p_v); 
        acc + p_v
    });
    let rand : f32 = rd.gen();
    let threshold : f32 = total * rand;
    let mut chosen_job = 0;
    let mut acc = 0.0;
    while acc <= threshold {
        acc = acc + pheromone_values[chosen_job];
        chosen_job = chosen_job + 1;
    }
    return jobs_left[chosen_job - 1]
}