const GENERATIONS : usize = 1000;
const PARTICLES : usize = 50;

use crate::job_list::{Jobs, Ordering};
use crate::particles::Swarm;

pub fn run<'a>(jobs : &'a Jobs) -> Ordering<'a> {
    let mut best_solution : Ordering = Ordering::random(&jobs);
    let mut best_time = best_solution.end_time();
    let mut swarm : Swarm = Swarm::random(PARTICLES);
    for i in 0..GENERATIONS {
        swarm.step();
        let solutions = swarm.make_solutions();
        for sol in solutions {
            if sol.end_time() < best_time {
                best_time = sol.end_time();
                best_solution = sol;
            }
        }
        println!("Gen {} : {}", i + 1, best_time);
    }
    return best_solution;
}