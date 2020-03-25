const EVAPORATION_RATE : f32 = 0.3;

use crate::job_list::Ordering;

pub struct PheromoneMatrix {
    n_tasks : usize,
    n_jobs : usize,
    n_nodes : usize,
    pheromones : Vec<f32>,
}

impl PheromoneMatrix {
    pub fn init(n_tasks : usize, n_jobs : usize) -> PheromoneMatrix {
        let n_nodes = n_tasks*n_jobs;
        let pheromones = vec![1.0; n_nodes*n_nodes];
        return PheromoneMatrix {n_jobs, n_tasks, n_nodes, pheromones}
    }

    pub fn get_pheromones(&self, job1 : usize, task1 : usize, job2 : usize, task2 : usize) -> f32 {
        let node1 = job1 * self.n_tasks * task1;
        let node2 = job2 * self.n_tasks * task2;
        return self.pheromones[self.n_nodes * node1 + node2];
    }

    pub fn update_edges(&mut self, sol : &Ordering) {
        // TODO : Implement
    } 
}
