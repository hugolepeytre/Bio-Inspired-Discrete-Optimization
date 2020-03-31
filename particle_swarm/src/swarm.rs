const GENERATIONS : usize = 300;
const PARTICLES : usize = 100;

use crate::job_list::{Jobs, Ordering};
use crate::particles::Swarm;

pub fn run<'a>(jobs : &'a Jobs) -> Ordering<'a> {
    let mut swarm : Swarm = Swarm::random(PARTICLES, jobs);
    for i in 0..GENERATIONS {
        swarm.step(jobs);
        println!("Gen {} : {}", i + 1, swarm.best_time());
    }
    return swarm.best_solution(jobs);
}