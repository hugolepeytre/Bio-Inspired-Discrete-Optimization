const EVAPORATION_RATE : f32 = 0.3;
const ALPHA : f32 = 10.0;

use crate::job_list::Ordering;

use std::cmp::{max, min};

pub struct PheromoneMatrix {
    n_tasks : usize,
    n_jobs : usize,
    n_nodes : usize,
    subgraph_size : usize,
    pheromones : Vec<f32>, // For each machine, we have just the amount of edges necessary
    // Indexed by lowest -> highest (edges are undirected). So a total of (n^2 - n)/2 edges per machine, m*(n^2 - n)/2 total
}

impl PheromoneMatrix {
    pub fn init(n_tasks : usize, n_jobs : usize) -> PheromoneMatrix {
        let n_nodes = n_tasks*n_jobs;
        let pheromones = vec![1.0; n_nodes*n_nodes];
        let subgraph_size = n_jobs * (n_jobs - 1)/2;
        return PheromoneMatrix {n_jobs, n_tasks, n_nodes, pheromones, subgraph_size}
    }

    pub fn get_pheromones(&self, task1 : usize, task2 : usize, machine_num : usize) -> f32 {
        let t1 = max(task1, task2);
        let t2 = min(task1, task2);
        let idx_main = t1 * self.n_jobs - (t1 * (t1 + 1)/2);
        let idx_sub = t2 - t1 - 1;
        let idx = idx_main + idx_sub;
        return self.pheromones[self.subgraph_size * machine_num + idx];
    }

    pub fn update_edges(&mut self, sol : &Ordering) {
        // TODO : Implement
    } 
}
