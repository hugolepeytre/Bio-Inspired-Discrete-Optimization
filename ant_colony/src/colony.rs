const GENERATIONS : usize = 5000;
const ANTS : usize = 100;

use crate::job_list::{Jobs, Ordering};
use crate::ants::PheromoneMatrix;

use std::usize::MAX;

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
    // TODO implement
    let solution : Vec<(usize, usize)> = (0..jobs.n_machines()).map(|n_machine| {
        let mut current_job = MAX;
        let jobs_left : Vec<usize> = (0..jobs.n_jobs()).collect();
        let mut job_order = Vec::new();
        while !jobs_left.is_empty() {
            current_job = choose_next_job(pheromones, current_job, &jobs_left);
            job_order.push(current_job);
            jobs_left.remove(current_job);
        }
        job_order
    }).flatten().collect();
    return Ordering::new(solution, jobs);
}

fn choose_next_job(pheromones : &PheromoneMatrix, curr_job : usize, jobs_left : &Vec<usize>) -> usize{
    // TODO : Implement
    return 0
}