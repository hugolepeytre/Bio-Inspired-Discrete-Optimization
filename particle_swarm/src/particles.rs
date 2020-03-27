use crate::job_list::Ordering;

use std::usize::MAX;

pub struct Swarm {
    particles : Vec<Particle>,
    best_known_pos : Vec<f32>,
    best_known_time : usize,
}

impl Swarm {
    pub fn random(size : usize) -> Swarm {
        let particles = (0..size).map(|_| Particle::random()).collect();
        let s = Swarm {particles, best_known_pos : Vec::new(), best_known_time : MAX};
        return s
    }

    pub fn step(&mut self) {
        // TODO : Implement
        // Make every particle do a step
        // Evaluate them
    }

    fn evaluate(part : &Particle) {

    }

    pub fn best_time(&self) -> usize {
        return self.best_known_time
    }

    pub fn best_solution<'a>(&self) -> Option<Ordering<'a>> {
        // TODO : have it return something
        None
    }
}

pub struct Particle {
    speed : f32,
    pos : Vec<f32>,
    self_best_pos : Vec<f32>,
    self_best_time : usize,
}

impl Particle {
    pub fn new(speed : f32, pos : Vec<f32>) -> Particle {
        return Particle {speed, pos}
    }

    pub fn random() -> Particle {
        // TODO : Implement
        return Self::new(0.0, Vec::new())
    }

    pub fn eval(&self) -> usize {
        // TODO : Implement
        return 0
    }
}