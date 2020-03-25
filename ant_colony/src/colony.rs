const GENERATIONS : usize = 5000;
const ANTS : usize = 100;

use crate::job_list::{Jobs, Ordering};
use crate::ants::PheromoneMatrix;

pub fn run(jobs : Jobs) -> Ordering {
    let mut best_solution : Ordering = Ordering::random(&jobs);
    let mut pheromones : PheromoneMatrix = PheromoneMatrix::init(jobs.n_machines(), jobs.n_jobs());
    for _ in 0..GENERATIONS {
        let solutions : Vec<_> = (0..ANTS).map(|_| construct_solution(&pheromones, &jobs)).collect();
        for s in solutions {
            pheromones.update_edges(&s);
            if best_solution.end_time() > s.end_time() {
                best_solution = s;
            }
        }
    }
    return best_solution;
}

fn construct_solution(pheromones : &PheromoneMatrix, jobs : &Jobs) -> Ordering {
    // TODO implement
    return Ordering::new(Vec::new(), jobs);
}