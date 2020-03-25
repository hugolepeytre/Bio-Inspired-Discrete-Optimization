pub struct Jobs {
    n_jobs : usize,
    n_machines : usize,
    processing_times : Vec<usize>, // Processing time of task j from job i : [i * machines + j]
    machine_numbers : Vec<usize>, // Same as above but machine number
    tasks_per_machines : Vec<usize>, // Which is machine i's task from job j (redundant but speeds up research)
}

impl Jobs {
    pub fn init(machines_and_times : Vec<(usize, usize)>, n_machines : usize, n_jobs : usize) -> Jobs {
        let mut processing_times = Vec::new();
        let mut machine_numbers = Vec::new();
        let mut tasks_per_machines = vec![0; n_jobs*n_machines];
        for (machine, duration) in machines_and_times {
            processing_times.push(duration);
            machine_numbers.push(machine);
        }
        for (i, &machine) in machine_numbers.iter().enumerate() {
            let task_number = i % n_jobs;
            let job_number = i / n_jobs;
            println!("Job {}'s task {} is on machine {}", job_number, task_number, machine);
            tasks_per_machines[machine * n_jobs + job_number] = task_number;
        }
        for &i in &tasks_per_machines {
            print!("{} ", i);
        }
        return Jobs {n_jobs, n_machines, processing_times, machine_numbers, tasks_per_machines}
    }

    pub fn n_jobs(&self) -> usize {
        return self.n_jobs
    }

    pub fn n_machines(&self) -> usize {
        return self.n_machines
    }
}

pub struct Ordering {
    tasks_order : Vec<(usize, usize)>, // A task is identified by a job number and a task number
    end_time : usize,
}

impl Ordering {
    pub fn new(tasks_order : Vec<(usize, usize)>, jobs : &Jobs) -> Ordering {
        let end_time = Self::eval(&tasks_order, jobs);
        return Ordering {tasks_order, end_time}
    }

    pub fn random(jobs : &Jobs) -> Ordering {
        let tasks_order : Vec<(usize, usize)> = (0..jobs.n_jobs()*jobs.n_machines()).map(|i| (i/jobs.n_jobs(), i%jobs.n_machines())).collect();
        return Self::new(tasks_order, jobs)
    }

    fn eval(tasks_order : &Vec<(usize, usize)>, jobs : &Jobs) -> usize {
        // TODO : Implement
        return 0
    }

    pub fn end_time(&self) -> usize {
        return self.end_time
    }

    pub fn output(&self) -> String {
        // TODO : Implement
        // Use method to generate start and end times matrix
        // One line for each machine
        //  Print a triplet for every task (task/job) start_time end_time 
        return String::from("")
    }

    fn generate_times(&self) {
        // TODO : Implement
    }
}