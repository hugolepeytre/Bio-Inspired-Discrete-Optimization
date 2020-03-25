const EVAPORATION_RATE : f32 = 0.8;
const PERSISTENCE_RATE : f32 = 1.0 - EVAPORATION_RATE;
const ALPHA : f32 = 1.0;
const MAX_PH : f32 = 25.0;
const PH_POW : f32 = 10.0;

use crate::job_list::Ordering;

use std::cmp::{max, min};
use std::usize::MAX;
use std::iter::once;

pub struct PheromoneMatrix {
    n_tasks : usize,
    n_nodes : usize,
    pheromones : Vec<f32>, // For each machine, we have just the amount of edges necessary
    // Indexed by lowest -> highest (edges are undirected). So a total of (n^2 - n)/2 edges per machine, m*(n^2 - n)/2 total
}

impl PheromoneMatrix {
    pub fn init(n_tasks : usize, n_jobs : usize) -> PheromoneMatrix {
        let n_nodes = n_jobs * n_tasks;
        let pheromones = vec![1.0; n_nodes * (n_nodes + 1)/2];
        return PheromoneMatrix {n_tasks, n_nodes, pheromones}
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

    pub fn update_edges(&mut self, sol : &Ordering, best_time : usize) {
        for i in 0..self.pheromones.len() {
            self.pheromones[i] = 1.0 + (self.pheromones[i] - 1.0) * PERSISTENCE_RATE;
        }
        let ord = sol.order_per_machines();
        let edges = once(&(MAX, MAX)).chain(ord.iter().take(ord.len() - 1)).zip(ord.iter());
        let pheromone_added = ALPHA * (2.0 * best_time as f32/sol.end_time() as f32).powf(PH_POW);
        for (&(job1, task1), &(job2, task2)) in edges {
            self.update_edge(job1, job2, task1, task2, pheromone_added);
        }
    }

    pub fn update_edge(&mut self, job1 : usize, job2 : usize, task1 : usize, task2 : usize, pheromone_added : f32) {
        if job1 == MAX {
            let node = job2 * self.n_tasks + task2;
            self.pheromones[node] = self.pheromones[node] + pheromone_added;
        }
        else {
            let node_num1 = job1 * self.n_tasks + task1;
            let node_num2 = job2 * self.n_tasks + task2;
            let n1 = min(node_num1, node_num2);
            let n2 = max(node_num1, node_num2);
            let idx_main = (n1 + 1) * self.n_nodes - (n1 * (n1 + 1)/2);
            let idx_sub = n2 - n1 - 1;
            let idx = idx_main + idx_sub;
            self.pheromones[idx] = minf(self.pheromones[idx] + pheromone_added, MAX_PH);
        }
    }
}

fn minf(a : f32, b : f32) -> f32 {
    return if a < b {a} else {b}
}