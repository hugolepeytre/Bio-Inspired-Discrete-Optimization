use crate::job_list::{Jobs, Ordering};
use crate::particles::Swarm;

pub fn run<'a>(jobs : &'a Jobs) -> Ordering<'a> {
    let mut swarm : Swarm = Swarm::random(jobs);
    swarm.run(jobs);
    return swarm.best_solution(jobs);
}