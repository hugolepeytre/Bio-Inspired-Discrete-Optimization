const EVAPORATION_RATE : f32 = 0.3;
const ALPHA : f32 = 10.0;

use crate::job_list::Ordering;

use std::cmp::{max, min};
use std::usize::MAX;

pub struct PheromoneMatrix {
    n_tasks : usize,
    n_jobs : usize,
    n_nodes : usize,
    pheromones : Vec<f32>, // For each machine, we have just the amount of edges necessary
    // Indexed by lowest -> highest (edges are undirected). So a total of (n^2 - n)/2 edges per machine, m*(n^2 - n)/2 total
}

impl PheromoneMatrix {
    pub fn init(n_tasks : usize, n_jobs : usize) -> PheromoneMatrix {
        let n_nodes = n_jobs * n_tasks;
        let pheromones = vec![1.0; n_nodes * (n_nodes + 1)/2];
        return PheromoneMatrix {n_jobs, n_tasks, n_nodes, pheromones}
    }

    pub fn get_pheromones(&self, job1 : usize, job2 : usize, task1 : usize, task2 : usize) -> f32 {
        if job1 == MAX {
            let node = job2 * self.n_tasks + task2;
            return self.pheromones[node]
        }
        else {
            let node_num1 = job1 * self.n_tasks + task1;
            let node_num2 = job2 * self.n_tasks + task2;
            let n1 = min(node_num1, node_num2);
            let n2 = max(node_num1, node_num2);
            let idx_main = (n1 + 1) * self.n_nodes - (n1 * (n1 + 1)/2);
            let idx_sub = n2 - n1 - 1;
            let idx = idx_main + idx_sub;
            return self.pheromones[idx];
        }
    }

    pub fn update_edges(&mut self, sol : &Ordering) {
        // TODO : Implement
    }
}
