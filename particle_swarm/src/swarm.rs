const GENERATIONS : usize = 1000;
const PARTICLES : usize = 50;

use crate::job_list::{Jobs, Ordering};
use crate::particles::Swarm;

pub fn run<'a>(jobs : &'a Jobs) -> Ordering<'a> {
    let mut swarm : Swarm = Swarm::random(PARTICLES);
    for i in 0..GENERATIONS {
        swarm.step();
        println!("Gen {} : {}", i + 1, swarm.best_time());
    }
    return swarm.best_solution().unwrap();
}