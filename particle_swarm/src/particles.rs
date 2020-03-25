use crate::job_list::Ordering;

pub struct Swarm {
    particles : Vec<Particle>,
}

impl Swarm {
    pub fn random(size : usize) -> Swarm {
        let particles = (0..size).map(|_| Particle::random()).collect();
        return Swarm {particles}
    }

    pub fn step(&mut self) {
        // TODO : Implement
    }

    pub fn make_solutions<'a>(&self) -> Vec<Ordering<'a>> {
        // TODO : Implement
        return Vec::new()
    }
}

pub struct Particle {
    speed : f32,
    pos : Vec<f32>,
}

impl Particle {
    pub fn new(speed : f32, pos : Vec<f32>) -> Particle {
        return Particle {speed, pos}
    }

    pub fn random() -> Particle {
        // TODO : Implement
        return Self::new(0.0, Vec::new())
    }
}